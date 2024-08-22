#include "esp_crt_bundle.h"
#include "esp_err.h"
#include "esp_http_client.h"
#include "esp_log.h"
#include "esp_wifi.h"
#include <fcntl.h>
#include <sys/param.h>

static esp_netif_t *wifi_if = NULL;
static const char *TAG = "production-https";
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

static void got_ip_handler(void *arg, esp_event_base_t event_base,
                           int32_t event_id, void *event_data) {
  ESP_LOGI(TAG, "Got IP address");
  xSemaphoreGive(IP_SEMPH);
}

esp_err_t connect_to_ap() {
  wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
  cfg.nvs_enable = 0;
  ESP_ERROR_CHECK(esp_wifi_init(&cfg));

  esp_netif_inherent_config_t esp_netif_config =
      ESP_NETIF_INHERENT_DEFAULT_WIFI_STA();
  wifi_if = esp_netif_create_wifi(WIFI_IF_STA, &esp_netif_config);
  esp_wifi_set_default_wifi_sta_handlers();

  ESP_ERROR_CHECK(esp_wifi_set_storage(WIFI_STORAGE_RAM));
  ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
  ESP_ERROR_CHECK(esp_wifi_start());

  IP_SEMPH = xSemaphoreCreateBinary();
  if (IP_SEMPH == NULL) {
    return ESP_ERR_NO_MEM;
  }

  wifi_config_t wifi_config = {
      .sta =
          {
              .ssid = SSID,
              .password = PASSWORD,
          },
  };
  ESP_LOGI(TAG, "Connecting to %s...", wifi_config.sta.ssid);
  esp_wifi_set_config(WIFI_IF_STA, &wifi_config);

  ESP_ERROR_CHECK(esp_event_handler_register(IP_EVENT, IP_EVENT_STA_GOT_IP,
                                             &got_ip_handler, NULL));

  esp_wifi_connect();

  xSemaphoreTake(IP_SEMPH, portMAX_DELAY);

  return ESP_OK;
}

static void stop_wifi() {
  ESP_LOGI(TAG, "Turning off wifi");
  ESP_ERROR_CHECK(esp_wifi_stop());
  ESP_ERROR_CHECK(esp_wifi_deinit());
  ESP_ERROR_CHECK(esp_wifi_clear_default_wifi_driver_and_handlers(wifi_if));
  esp_netif_destroy(wifi_if);
  wifi_if = NULL;
}

static void https_with_url(void *pvParameters) {
  while (true) {
    ESP_ERROR_CHECK(connect_to_ap());

    esp_http_client_config_t config = {
        .url = "https://ncona.com/about-me/",
        .event_handler = http_event_handler,
        .crt_bundle_attach = esp_crt_bundle_attach,
    };
    esp_http_client_handle_t client = esp_http_client_init(&config);
    esp_err_t err = esp_http_client_perform(client);

    if (err == ESP_OK) {
      int status_code = esp_http_client_get_status_code(client);
      if (status_code != 200) {
        ESP_LOGI(TAG, "Got %d code", status_code);
      }
    } else {
        ESP_LOGE(TAG, "Error with https request: %s", esp_err_to_name(err));
    }

    ESP_ERROR_CHECK(esp_http_client_cleanup(client));
    stop_wifi();
    vTaskDelay(60000 / portTICK_PERIOD_MS);
  }
}

extern "C" void app_main() {
  ESP_ERROR_CHECK(esp_netif_init());
  ESP_ERROR_CHECK(esp_event_loop_create_default());
  xTaskCreate(&https_with_url, "https_with_url", 8192, NULL, 5, NULL);
}
