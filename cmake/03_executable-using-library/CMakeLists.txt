cmake_minimum_required(VERSION 3.20)
project(ExecutableWithLibrary)

add_library(math src/math.c)
add_executable(main src/main.c)

target_link_libraries(main PRIVATE math)
