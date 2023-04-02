cmake_minimum_required(VERSION 3.0.0)
project(cpp_rust_demo VERSION 0.1.0)

message("input build type ${CMAKE_BUILD_TYPE}, custom build type ${BUILD_TYPE}")

if(NOT DEFINED CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE ${BUILD_TYPE})
endif ()

message("build type ${CMAKE_BUILD_TYPE}")

add_subdirectory(simple_ffi)
add_subdirectory(cpp_demo)
