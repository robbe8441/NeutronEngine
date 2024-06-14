cmake_minimum_required(VERSION 3.27)
project(vulkan_bindings C)

set(SOURCES
    src/lib.c
)

add_library(vulkan STATIC ${SOURCES})

install(TARGETS vulkan DESTINATION .)
