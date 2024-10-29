#include "library.hpp"

#include <iostream>

#include <esp_event.h>

void replace(std::string &in, const char f, const char r) {
  for (long unsigned int i = 0; i < in.length(); i++) {
    if (in[i] == f) {
      in[i] = r;
    }
  }
}

std::unordered_map<std::string, std::string>
parse_query_string(const std::string &line) {
  std::unordered_map<std::string, std::string> dictionary;

  int start = line.find_first_of("?") + 1;
  int end = line.find_first_of(" ", start);
  std::string query_string = line.substr(start, end);
  replace(query_string, '+', ' ');

  long unsigned int current_start = 0;
  while (current_start < query_string.length()) {
    int current_end = query_string.find_first_of("&", current_start);
    if (current_end == -1) {
      current_end = query_string.length();
    }

    std::string current_pair =
        query_string.substr(current_start, current_end - current_start);
    int equalPos = current_pair.find_first_of("=");
    if (equalPos != -1) {
      // If there is no equal sign, we skip adding it to the result
      std::string key = current_pair.substr(0, equalPos);
      std::string value =
          current_pair.substr(equalPos + 1, current_pair.length());
      dictionary[key] = value;
    }

    current_start = current_end + 1;
  }

  return dictionary;
}

esp_err_t event_loop() {
  return esp_event_loop_create_default();
}
