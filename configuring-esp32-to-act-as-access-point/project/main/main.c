#include "esp_http_server.h"
#include "esp_log.h"
#include "esp_wifi.h"
#include "lwip/sockets.h"

#define EXAMPLE_ESP_WIFI_SSID "my-esp32-ssid"
#define EXAMPLE_ESP_WIFI_PASS "APassword"
#define EXAMPLE_MAX_STA_CONN 4

static const char *TAG = "example";
extern const char hello_start[] asm("_binary_hello_html_start");
extern const char hello_end[] asm("_binary_hello_html_end");

struct dns_answer {
  uint16_t ptr_offset;
  uint16_t type;
  uint16_t class;
  uint32_t ttl;
  uint16_t addr_len;
  uint32_t ip_addr;
} __attribute__((__packed__));

struct dns_server_handle {
  TaskHandle_t task;
};

void dns_server_task(void *pvParameters) {
  char rx_buffer[128];

  while (true) {
    struct sockaddr_in dest_addr;
    dest_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    dest_addr.sin_family = AF_INET;
    dest_addr.sin_port = htons(53);

    int sock = socket(AF_INET, SOCK_DGRAM, IPPROTO_IP);
    if (sock < 0) {
      ESP_LOGE(TAG, "Unable to create socket: errno %d", errno);
      break;
    }

    int err = bind(sock, (struct sockaddr *)&dest_addr, sizeof(dest_addr));
    if (err < 0) {
      ESP_LOGE(TAG, "Socket unable to bind: errno %d", errno);
    }

    while (true) {
      ESP_LOGI(TAG, "Waiting for data");
      struct sockaddr_in source_addr;
      socklen_t addrlen = sizeof(source_addr);
      int len = recvfrom(sock, rx_buffer, sizeof(rx_buffer) - 1, 0,
                         (struct sockaddr *)&source_addr, &addrlen);

      // Error occurred during receiving
      if (len < 0) {
        ESP_LOGE(TAG, "recvfrom failed: errno %d", errno);
        close(sock);
        break;
      } else {
        char dns_response[256] = {};
        memcpy(dns_response, rx_buffer, len);
        dns_response[2] |= (1 << 7);
        dns_response[6] = dns_response[4];
        dns_response[7] = dns_response[5];

        int dns_response_len = sizeof(struct dns_answer) + len;

        // Move to the start of the questions section. Since the DNS header is 12 bytes,
        // we just need to move 12 bytes from the start of the response
        char *qn_ptr = dns_response + 12;

        // Move to the end of the request
        char *ans_ptr = dns_response + len;

        // Cast the memory to our dns_aswer type
        struct dns_answer *answer = (struct dns_answer *)ans_ptr;

        // 0x0c is the same as 1100_0000. We use | to make sure set those bits on the
        // pointer. To convert to network order, call htons
        answer->ptr_offset = htons(0xc000 | (qn_ptr - dns_response));

        char *qn_type_ptr = qn_ptr;
        while (qn_type_ptr[0] != 0x0) {
          qn_type_ptr++;
        }
        qn_type_ptr++;
        answer->type = *(uint16_t *)qn_type_ptr;
        qn_type_ptr += 2;
        answer->class = *(uint16_t *)qn_type_ptr;

        answer->ttl = htonl(300);

        // We are using IPv4 so we know it'll always be 4 bytes
        answer->addr_len = htons(4);

        // Get the IP information from default AP device
        esp_netif_ip_info_t ip_info;
        esp_netif_get_ip_info(esp_netif_get_handle_from_ifkey("WIFI_AP_DEF"), &ip_info);
        answer->ip_addr = ip_info.ip.addr;

        if (dns_response_len <= 0) {
          ESP_LOGE(TAG, "Failed to prepare a DNS dns_response");
        } else {
          int err =
              sendto(sock, dns_response, dns_response_len, 0, (struct sockaddr *)&source_addr,
                     sizeof(source_addr));
          if (err < 0) {
            ESP_LOGE(TAG, "Error occurred during sending: errno %d", errno);
            break;
          }
        }
      }
    }

    if (sock != -1) {
      ESP_LOGE(TAG, "Shutting down socket");
      shutdown(sock, 0);
      close(sock);
    }
  }
}

void start_dns_server() {
  struct dns_server_handle *handle =
      calloc(1, sizeof(struct dns_server_handle));
  xTaskCreate(dns_server_task, "dns_server", 4096, handle, 5, &handle->task);
}

static void wifi_init_softap(void) {
  wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
  cfg.nvs_enable = 0;
  ESP_ERROR_CHECK(esp_wifi_init(&cfg));

  ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_AP));
  wifi_config_t wifi_config = {
      .ap = {.ssid = EXAMPLE_ESP_WIFI_SSID,
             .ssid_len = strlen(EXAMPLE_ESP_WIFI_SSID),
             .password = EXAMPLE_ESP_WIFI_PASS,
             .max_connection = EXAMPLE_MAX_STA_CONN,
             .authmode = WIFI_AUTH_WPA_WPA2_PSK},
  };
  ESP_ERROR_CHECK(esp_wifi_set_config(ESP_IF_WIFI_AP, &wifi_config));
  ESP_ERROR_CHECK(esp_wifi_start());

  esp_netif_ip_info_t ip_info;
  esp_netif_get_ip_info(esp_netif_get_handle_from_ifkey("WIFI_AP_DEF"),
                        &ip_info);

  char ip_addr[16];
  inet_ntoa_r(ip_info.ip.addr, ip_addr, 16);
  ESP_LOGI(TAG, "Set up softAP with IP: %s", ip_addr);
}

static esp_err_t hello_handler(httpd_req_t *req) {
  const uint32_t hello_len = hello_end - hello_start;

  ESP_LOGI(TAG, "Serve hello");
  httpd_resp_set_type(req, "text/html");
  httpd_resp_send(req, hello_start, hello_len);

  return ESP_OK;
}

static const httpd_uri_t hello = {
    .uri = "/", .method = HTTP_GET, .handler = hello_handler};

esp_err_t http_404_error_handler(httpd_req_t *req, httpd_err_code_t err) {
  httpd_resp_set_status(req, "302 Temporary Redirect");
  httpd_resp_set_hdr(req, "Location", "/");

  // iOS requires content in the response to detect a captive portal, simply
  // redirecting is not sufficient.
  httpd_resp_send(req, "Redirect to the captive portal", HTTPD_RESP_USE_STRLEN);

  ESP_LOGI(TAG, "Redirecting to hello");
  return ESP_OK;
}

static void start_webserver(void) {
  httpd_handle_t server = NULL;
  httpd_config_t config = HTTPD_DEFAULT_CONFIG();
  ESP_LOGI(TAG, "Starting server on port: '%d'", config.server_port);
  ESP_ERROR_CHECK(httpd_start(&server, &config));

  // Set URI handlers
  ESP_LOGI(TAG, "Registering URI handlers");
  httpd_register_uri_handler(server, &hello);
  httpd_register_err_handler(server, HTTPD_404_NOT_FOUND,
                             http_404_error_handler);
}

void app_main(void) {
  ESP_ERROR_CHECK(esp_netif_init());
  ESP_ERROR_CHECK(esp_event_loop_create_default());

  esp_netif_create_default_wifi_ap();

  wifi_init_softap();
  start_webserver();
  start_dns_server();
}
