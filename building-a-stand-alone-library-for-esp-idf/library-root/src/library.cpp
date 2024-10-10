#include "library.hpp"

#include "esp_log.h"

const char *TAG = "example";

void Library::do_something() {
  ESP_LOGI(TAG, "hello");
}
