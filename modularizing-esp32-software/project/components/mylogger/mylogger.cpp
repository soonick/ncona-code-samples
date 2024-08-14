#include "mylogger.hpp"

#include "esp_log.h"

void log(const char* in) {
  esp_log_write(ESP_LOG_INFO, "ignored", "%s: %s\n", "tag", in);
}
