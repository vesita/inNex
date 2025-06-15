cmake_minimum_required(VERSION 3.20)

PROJECT(inNex)

set(CMAKE_CXX_STANDARD 20)

include_directories(inNex PRIVATE ${PROJECT_SOURCE_DIR}/hdr)

file(GLOB origins src/*.cpp)

add_executable(inNex
    main.cpp
    ${origins}
)

