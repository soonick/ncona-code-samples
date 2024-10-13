#pragma once

#include <unordered_map>
#include <string>

std::unordered_map<std::string, std::string> parse_query_string(const std::string&);
