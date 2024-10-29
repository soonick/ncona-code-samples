cmake_minimum_required(VERSION 3.27.4)
project(LibraryTest
  VERSION 0.1.0
  LANGUAGES CXX)

include_directories(external/catch2)
include_directories(../include)
include_directories(mock)

FILE(GLOB MOCK_SRCS mock/*.cpp)

set(TESTING_SRCS
  ../src/library.cpp
)

set(TEST_TARGET_SRCS
  src/test-main.cpp
)

add_compile_options(-Wall -Wextra -Wpedantic -Werror)

add_executable(
  test
  ${MOCK_SRCS}
  ${TEST_TARGET_SRCS}
  ${TESTING_SRCS}
)
