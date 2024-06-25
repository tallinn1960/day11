cmake_minimum_required(VERSION 3.22)
project(day11Swift Swift)

if (CMAKE_SYSTEM_NAME)
    if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
        if(CMAKE_SYSTEM_PROCESSOR)
            if(CMAKE_SYSTEM_PROCESSOR STREQUAL "arm64")
                set(CMAKE_Swift_FLAGS "${CMAKE_Swift_FLAGS} -target arm64-apple-macosx14")
            elseif(CMAKE_SYSTEM_PROCESSOR STREQUAL "x86_64")
                set(CMAKE_Swift_FLAGS "${CMAKE_Swift_FLAGS} -target x86_64-apple-macosx14")
            endif()
        endif(CMAKE_SYSTEM_PROCESSOR)
    endif()
endif()

add_custom_command(OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/day11.h
    DEPENDS day11Swift
    COMMENT "Generating Swift header"
)
# this is for c++ code to depend on the swift header
add_custom_target(trap_swift_header DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/day11.h)
add_library(day11Swift STATIC Swift/Sources/Day11/Day11.swift)
target_compile_options(day11Swift PRIVATE "SHELL:-cxx-interoperability-mode=default -emit-objc-header-path ${CMAKE_CURRENT_BINARY_DIR}/day11.h -module-name day11")
target_include_directories(day11Swift PUBLIC ${CMAKE_CURRENT_BINARY_DIR})
# this is for rust to find the swift library
install(TARGETS day11Swift DESTINATION .)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/day11.h DESTINATION .)
