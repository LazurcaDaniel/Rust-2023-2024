# Install script for directory: C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk

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
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/lib/cfltk.lib")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/lib" TYPE STATIC_LIBRARY FILES "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/cfltk.lib")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_box.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_browser.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_button.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_dialog.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_draw.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_enums.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_group.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_image.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_input.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_lock.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_macros.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_menu.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_misc.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_prefs.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_printer.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_surface.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_table.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_text.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_tree.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_utils.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_valuator.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_widget.h;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_widget.hpp;C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/include/cfltk" TYPE FILE FILES
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_box.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_browser.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_button.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_dialog.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_draw.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_enums.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_group.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_image.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_input.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_lock.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_macros.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_menu.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_misc.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_prefs.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_printer.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_surface.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_table.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_text.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_tree.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_utils.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_valuator.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_widget.h"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_widget.hpp"
    "C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/include/cfl_window.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfig.cmake"
         "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/CMakeFiles/Export/8669349c16bd612bfdf74e820fda948e/cfltkConfig.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/CMakeFiles/Export/8669349c16bd612bfdf74e820fda948e/cfltkConfig.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfig-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    file(INSTALL DESTINATION "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/CMakeFiles/Export/8669349c16bd612bfdf74e820fda948e/cfltkConfig-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/cmake/cfltk" TYPE FILE FILES "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/cfltkConfigVersion.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/pkgconfig/cfltk.pc")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/share/pkgconfig" TYPE FILE FILES "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/cfltk.pc")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
