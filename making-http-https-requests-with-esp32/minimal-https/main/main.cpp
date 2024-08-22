#include "esp_crt_bundle.h"
#include "esp_http_client.h"
#include "esp_log.h"
#include "esp_wifi.h"
#include <sys/param.h>

static const char *TAG = "minimal-https";
static SemaphoreHandle_t IP_SEMPH = NULL;
#define SSID "<YOUR_SSID>"
#define PASSWORD "<YOUR_PASSWORD>"

esp_err_t http_event_handler(esp_http_client_event_t *evt) {
  static char *output_buffer;
  static int output_len;
  switch (evt->event_id) {
  case HTTP_EVENT_ERROR:
    ESP_LOGI(TAG, "HTTP_EVENT_ERROR");
    break;
  case HTTP_EVENT_ON_CONNECTED:
    ESP_LOGI(TAG, "HTTP_EVENT_ON_CONNECTED");
    break;
  case HTTP_EVENT_HEADER_SENT:
    ESP_LOGI(TAG, "HTTP_EVENT_HEADER_SENT");
    break;
  case HTTP_EVENT_ON_HEADER:
    ESP_LOGI(TAG, "HTTP_EVENT_ON_HEADER, key=%s, value=%s", evt->header_key,
             evt->header_value);
    break;
  case HTTP_EVENT_ON_DATA: {
    ESP_LOGI(TAG, "HTTP_EVENT_ON_DATA, len=%d", evt->data_len);
    int copy_len = 0;
    int content_len = esp_http_client_get_content_length(evt->client);
    if (output_buffer == NULL) {
      // We initialize output_buffer with 0 because it is used by strlen() and
      // similar functions therefore should be null terminated.
      output_buffer = (char *)calloc(content_len + 1, sizeof(char));
      output_len = 0;
      if (output_buffer == NULL) {
        ESP_LOGE(TAG, "Failed to allocate memory for output buffer");
        return ESP_FAIL;
      }
    }
    copy_len = MIN(evt->data_len, (content_len - output_len));
    if (copy_len) {
      memcpy(output_buffer + output_len, evt->data, copy_len);
    }
    output_len += copy_len;
    break;
  }
  case HTTP_EVENT_ON_FINISH:
    ESP_LOGI(TAG, "HTTP_EVENT_ON_FINISH");
    if (output_buffer != NULL) {
      ESP_LOGI(TAG, "%s", output_buffer);
      free(output_buffer);
      output_buffer = NULL;
    }
    output_len = 0;
    break;
  case HTTP_EVENT_DISCONNECTED:
    ESP_LOGI(TAG, "HTTP_EVENT_DISCONNECTED");
    if (output_buffer != NULL) {
      free(output_buffer);
      output_buffer = NULL;
    }
    output_len = 0;
    break;
  case HTTP_EVENT_REDIRECT:
    ESP_LOGI(TAG, "HTTP_EVENT_REDIRECT");
    break;
  }
  return ESP_OK;
}

static void https_with_url() {
  esp_http_client_config_t config = {
      .url = "https://ncona.com/about-me/",
      .event_handler = http_event_handler,
      .crt_bundle_attach = esp_crt_bundle_attach,
  };
  esp_http_client_handle_t client = esp_http_client_init(&config);
  esp_http_client_perform(client);
}

static void example_handler_on_sta_got_ip(void *arg,
                                          esp_event_base_t event_base,
                                          int32_t event_id, void *event_data) {
  ip_event_got_ip_t *event = (ip_event_got_ip_t *)event_data;
  ESP_LOGI(TAG, "Got IPv4 event: Interface \"%s\" address: " IPSTR,
           esp_netif_get_desc(event->esp_netif), IP2STR(&event->ip_info.ip));
  xSemaphoreGive(IP_SEMPH);
}

esp_err_t connectToAp() {
  wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
  cfg.nvs_enable = 0;
  esp_wifi_init(&cfg);

  esp_netif_inherent_config_t esp_netif_config =
      ESP_NETIF_INHERENT_DEFAULT_WIFI_STA();
  esp_netif_create_wifi(WIFI_IF_STA, &esp_netif_config);
  esp_wifi_set_default_wifi_sta_handlers();

  esp_wifi_set_storage(WIFI_STORAGE_RAM);
  esp_wifi_set_mode(WIFI_MODE_STA);
  esp_wifi_start();

  wifi_config_t wifi_config = {
      .sta =
          {
              .ssid = SSID,
              .password = PASSWORD,
          },
  };
  ESP_LOGI(TAG, "Connecting to %s...", wifi_config.sta.ssid);
  esp_wifi_set_config(WIFI_IF_STA, &wifi_config);
  esp_wifi_connect();

  return ESP_OK;
}

extern "C" void app_main() {
  esp_netif_init();
  esp_event_loop_create_default();
  connectToAp();
  vTaskDelay(10000 / portTICK_PERIOD_MS);
  https_with_url();
  vTaskDelay(10000 / portTICK_PERIOD_MS);
}
