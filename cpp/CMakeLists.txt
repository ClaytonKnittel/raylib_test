cmake_minimum_required(VERSION 3.18)

project(raylib_test)

find_package(raylib 4.0 REQUIRED)

file(GLOB_RECURSE CC_SRC
  "${PROJECT_SOURCE_DIR}/src/*.cc"
)

add_executable(raylib_test ${CC_SRC})

target_link_libraries(raylib_test raylib)

if (APPLE)
  target_link_libraries(raylib_test "-framework IOKit")
  target_link_libraries(raylib_test "-framework Cocoa")
  target_link_libraries(raylib_test "-framework OpenGL")
endif()

