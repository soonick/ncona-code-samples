#define CATCH_CONFIG_MAIN
#include <catch.hpp>

#include "library.hpp"

TEST_CASE("parse_query_string") {
  SECTION("Multiple key values") {
    std::unordered_map<std::string, std::string> actual =
        parse_query_string("hello?abc=1&qwer=world&onemore=yesyes");
    REQUIRE(actual.size() == 3);
    REQUIRE(actual.at("abc") == "1");
    REQUIRE(actual.at("qwer") == "world");
    REQUIRE(actual.at("onemore") == "yesyes");
  }
}

TEST_CASE("event_loop") {
  SECTION("Returns ESP_OK") {
    REQUIRE(event_loop() == ESP_OK);
  }
}
