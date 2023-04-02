cmake_minimum_required(VERSION 3.0.0)
# project(cpp_demo VERSION 0.1.0)

get_target_property(RS_DIR simple_ffi LOCATION)

add_executable(cpp_demo main.cpp)

target_link_libraries(cpp_demo ${RS_DIR}/simple_ffi.dll.lib)
add_dependencies(cpp_demo simple_ffi)
