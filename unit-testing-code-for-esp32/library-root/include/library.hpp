#pragma once

#include <unordered_map>
#include <string>

#include <esp_err.h>

std::unordered_map<std::string, std::string> parse_query_string(const std::string&);
esp_err_t event_loop();
