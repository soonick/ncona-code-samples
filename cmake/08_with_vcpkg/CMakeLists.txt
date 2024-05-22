cmake_minimum_required(VERSION 3.28.3)
project(cmake-with-vcpkg CXX)

find_package(effolkronium_random CONFIG REQUIRED)

add_executable(${PROJECT_NAME} src/main.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE effolkronium_random)
