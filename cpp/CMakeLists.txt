cmake_minimum_required(VERSION 3.22)
project(day11cpp CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(day11cpp STATIC day11cpp.cpp)
install(TARGETS day11cpp DESTINATION .)
