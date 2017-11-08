cmake_minimum_required (VERSION 2.6)
project (pISO)

include_directories(
  "${PROJECT_SOURCE_DIR}/include"
  "${PROJECT_SOURCE_DIR}/CppGPIO/include")

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")

# For pthread
find_package (Threads)

# Build cppgpio
file(GLOB CPPGPIO_SRC_FILES ${PROJECT_SOURCE_DIR}/CppGPIO/src/*.cpp)
add_library(cppgpio STATIC ${CPPGPIO_SRC_FILES})

# json-cpp
find_package(PkgConfig REQUIRED)
pkg_check_modules(JSONCPP jsoncpp)
link_libraries(${JSONCPP_LIBRARIES})

# Build pISO
file(GLOB SRC_FILES ${PROJECT_SOURCE_DIR}/src/*.cpp)
add_executable(pISO ${SRC_FILES})

target_link_libraries (pISO
  cppgpio
  ${JSONCPP_LIBRARIES}
  ${CMAKE_THREAD_LIBS_INIT}
  -lwiringPi)

install (TARGETS pISO DESTINATION bin)
