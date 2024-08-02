#include "driver/gpio.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "example";

#define BLINK_GPIO GPIO_NUM_2

static uint8_t s_led_state = 0;

void app_main(void) {
  gpio_reset_pin(BLINK_GPIO);
  gpio_set_direction(BLINK_GPIO, GPIO_MODE_OUTPUT);

  while (1) {
    ESP_LOGI(TAG, "Turning the LED %s!", s_led_state == true ? "ON" : "OFF");
    gpio_set_level(BLINK_GPIO, s_led_state);
    s_led_state = !s_led_state;
    vTaskDelay(3000 / portTICK_PERIOD_MS); // Blink every 3 seconds
  }
}
