cmake_minimum_required(VERSION 3.20)
project(Math CXX)
add_library(Math src/math.cpp)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/heads/main.zip
)
FetchContent_MakeAvailable(googletest)

enable_testing()
add_executable(
  MathTest
  test/math_test.cpp
)
target_link_libraries(
  MathTest
  PRIVATE
  GTest::gtest_main
  Math
)
include(GoogleTest)
gtest_discover_tests(MathTest)
