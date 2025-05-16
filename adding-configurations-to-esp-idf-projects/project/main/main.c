#include "esp_log.h"
#include "freertos/FreeRTOS.h"

static const char *TAG = "project";

void app_main(void) {
  while (1) {
#ifdef CONFIG_BT_ENABLED
    ESP_LOGI(TAG, "BT enabled: %i", CONFIG_BT_ENABLED);
#else
    ESP_LOGI(TAG, "BT enabled: 0");
#endif
    ESP_LOGI(TAG, "The name: %s", CONFIG_PERSON_NAME);
    vTaskDelay(3000 / portTICK_PERIOD_MS); // Print every 3 seconds
  }
}
