# Install script for directory: C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk/jpeg

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk_jpeg.lib")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/FL/images" TYPE FILE FILES
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk/jpeg/jconfig.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk/jpeg/jerror.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk/jpeg/jmorecfg.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk/jpeg/jpeglib.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk/jpeg/fltk_jpeg_prefix.h"
    )
endif()
