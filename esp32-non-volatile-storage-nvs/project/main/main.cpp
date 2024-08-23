#include "esp_log.h"
#include "nvs_flash.h"
#include "nvs_handle.hpp"

#include <cstdint>
static const char *TAG = "nvs-example";

extern "C" void app_main(void) {
  esp_err_t err = nvs_flash_init();
  if (err == ESP_ERR_NVS_NO_FREE_PAGES ||
      err == ESP_ERR_NVS_NEW_VERSION_FOUND) {
    ESP_ERROR_CHECK(nvs_flash_erase());
    err = nvs_flash_init();
  }
  ESP_ERROR_CHECK(err);

  std::unique_ptr<nvs::NVSHandle> handle =
      nvs::open_nvs_handle("my_namespace", NVS_READWRITE, &err);
  ESP_ERROR_CHECK(err);

  uint32_t my_value = 0;
  err = handle->get_item("my_key", my_value);
  switch (err) {
  case ESP_OK:
    ESP_LOGI(TAG, "Value for my_key is: %" PRIu32, my_value);
    break;
  case ESP_ERR_NVS_NOT_FOUND:
    ESP_LOGI(TAG, "Key my_key doesn't exist in NVS");
    break;
  default:
    ESP_ERROR_CHECK(err);
  }

  my_value++;
  err = handle->set_item("my_key", my_value);
  ESP_ERROR_CHECK(err);
  err = handle->commit();
  ESP_ERROR_CHECK(err);
  ESP_LOGI(TAG, "Int written to NVS");

  std::unique_ptr<char[]> my_string = std::make_unique<char[]>(100);
  err = handle->get_string("my_string_key", my_string.get(), 100);
  switch (err) {
  case ESP_OK:
    ESP_LOGI(TAG, "Value for my_string_key is: %s", my_string.get());
    break;
  case ESP_ERR_NVS_NOT_FOUND:
    ESP_LOGI(TAG, "Key my_string_key doesn't exist in NVS");
    break;
  default:
    ESP_ERROR_CHECK(err);
  }

  const char *new_string = "Hello world";
  err = handle->set_string("my_string_key", new_string);
  ESP_ERROR_CHECK(err);
  err = handle->commit();
  ESP_ERROR_CHECK(err);
  ESP_LOGI(TAG, "String written to NVS");
}
