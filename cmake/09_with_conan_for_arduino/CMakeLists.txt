cmake_minimum_required(VERSION 3.28.3)
project(cmake-with-conan CXX)

set(CMAKE_C_COMPILER_FORCED TRUE)
set(CMAKE_CXX_COMPILER_FORCED TRUE)

add_executable(${PROJECT_NAME} src/main.cpp)
target_link_arduino_libraries(${PROJECT_NAME} PRIVATE core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(${PROJECT_NAME})
