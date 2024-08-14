#include "esp_log.h"
#include "freertos/FreeRTOS.h"

#include "animal.hpp"

extern "C" void app_main() {
  while (true) {
    Animal dog = Animal(number::ONE);
    dog.talk();
  }
}
