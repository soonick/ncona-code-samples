#include <cstdint>

#include <driver/gpio.h>
#include <esp_log.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>

#define OUT_PIN GPIO_NUM_18
#define INTERRUPT_PIN GPIO_NUM_19

const char* TAG = "interrupts-example";

// It's not safe to log from an ISR, so we use a queue
static QueueHandle_t queue;

static void interrupt_handler(void *args) {
  char c = 1;
  xQueueSendFromISR(queue, &c, nullptr);
}

// Prints a log for each task in the queue, as quickly as possible
void queue_task(void *params) {
  char c;
  while (true) {
    if (xQueueReceive(queue, &c, portMAX_DELAY)) {
      ESP_LOGI(TAG, "Interrupt triggered");
    }
  }
}

extern "C" void app_main() {
  // Initialize the queue
  queue = xQueueCreate(10, sizeof(uint32_t));

  // Start task
  xTaskCreate(queue_task, "queue_task", 2048, NULL, 10, NULL);

  // Configure output pin
  gpio_config_t out_config = {
      .pin_bit_mask = 1ULL << OUT_PIN,
      .mode = GPIO_MODE_OUTPUT,
      .pull_up_en = GPIO_PULLUP_DISABLE,
      .pull_down_en = GPIO_PULLDOWN_DISABLE,
      .intr_type = GPIO_INTR_DISABLE,
  };
  gpio_config(&out_config);

  // Configure interrupt pin
  gpio_config_t interrupt_config = {
      .pin_bit_mask = 1ULL << INTERRUPT_PIN,
      .mode = GPIO_MODE_INPUT,
      .pull_up_en = GPIO_PULLUP_ENABLE,
      .pull_down_en = GPIO_PULLDOWN_DISABLE,
      .intr_type = GPIO_INTR_ANYEDGE,
  };
  gpio_config(&interrupt_config);

  // Set up the ISR
  gpio_install_isr_service(0);
  gpio_isr_handler_add(INTERRUPT_PIN, interrupt_handler, nullptr);

  // Toggle the output every second
  uint32_t output_level = 0;
  while (true) {
    vTaskDelay(3000 / portTICK_PERIOD_MS);
    ESP_LOGI(TAG, "Toggling OUT_PIN");
    gpio_set_level(OUT_PIN, output_level);
    output_level ^= 1;
  }
}
