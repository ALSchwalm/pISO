cmake_minimum_required (VERSION 2.6)
project (USBMultitool)

include_directories(
  "${PROJECT_SOURCE_DIR}/include"
  "${PROJECT_SOURCE_DIR}/CppGPIO/include")

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")

# For pthread
find_package (Threads)

# Build cppgpio
file(GLOB CPPGPIO_SRC_FILES ${PROJECT_SOURCE_DIR}/CppGPIO/src/*.cpp)
add_library(cppgpio ${CPPGPIO_SRC_FILES})

# Build multitool
file(GLOB SRC_FILES ${PROJECT_SOURCE_DIR}/src/*.cpp)
add_executable(multitool ${SRC_FILES})

target_link_libraries (multitool lvm2app cppgpio  ${CMAKE_THREAD_LIBS_INIT})
