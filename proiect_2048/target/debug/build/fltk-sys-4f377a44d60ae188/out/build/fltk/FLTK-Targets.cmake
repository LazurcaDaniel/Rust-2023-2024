# Generated by CMake

if("${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION}" LESS 2.8)
   message(FATAL_ERROR "CMake >= 2.8.0 required")
endif()
if(CMAKE_VERSION VERSION_LESS "2.8.3")
   message(FATAL_ERROR "CMake >= 2.8.3 required")
endif()
cmake_policy(PUSH)
cmake_policy(VERSION 2.8.3...3.24)
#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Protect against multiple inclusion, which would fail when already imported targets are added once more.
set(_cmake_targets_defined "")
set(_cmake_targets_not_defined "")
set(_cmake_expected_targets "")
foreach(_cmake_expected_target IN ITEMS fltk_z fltk_jpeg fltk_png fltk fltk_forms fltk_images)
  list(APPEND _cmake_expected_targets "${_cmake_expected_target}")
  if(TARGET "${_cmake_expected_target}")
    list(APPEND _cmake_targets_defined "${_cmake_expected_target}")
  else()
    list(APPEND _cmake_targets_not_defined "${_cmake_expected_target}")
  endif()
endforeach()
unset(_cmake_expected_target)
if(_cmake_targets_defined STREQUAL _cmake_expected_targets)
  unset(_cmake_targets_defined)
  unset(_cmake_targets_not_defined)
  unset(_cmake_expected_targets)
  unset(CMAKE_IMPORT_FILE_VERSION)
  cmake_policy(POP)
  return()
endif()
if(NOT _cmake_targets_defined STREQUAL "")
  string(REPLACE ";" ", " _cmake_targets_defined_text "${_cmake_targets_defined}")
  string(REPLACE ";" ", " _cmake_targets_not_defined_text "${_cmake_targets_not_defined}")
  message(FATAL_ERROR "Some (but not all) targets in this export set were already defined.\nTargets Defined: ${_cmake_targets_defined_text}\nTargets not yet defined: ${_cmake_targets_not_defined_text}\n")
endif()
unset(_cmake_targets_defined)
unset(_cmake_targets_not_defined)
unset(_cmake_expected_targets)


# Create imported target fltk_z
add_library(fltk_z STATIC IMPORTED)

# Create imported target fltk_jpeg
add_library(fltk_jpeg STATIC IMPORTED)

# Create imported target fltk_png
add_library(fltk_png STATIC IMPORTED)

set_target_properties(fltk_png PROPERTIES
  INTERFACE_LINK_LIBRARIES "fltk_z"
)

# Create imported target fltk
add_library(fltk STATIC IMPORTED)

set_target_properties(fltk PROPERTIES
  INTERFACE_LINK_LIBRARIES "comctl32;ws2_32;gdiplus"
)

# Create imported target fltk_forms
add_library(fltk_forms STATIC IMPORTED)

set_target_properties(fltk_forms PROPERTIES
  INTERFACE_LINK_LIBRARIES "fltk"
)

# Create imported target fltk_images
add_library(fltk_images STATIC IMPORTED)

set_target_properties(fltk_images PROPERTIES
  INTERFACE_LINK_LIBRARIES "fltk;fltk_jpeg;fltk_z;fltk_png"
)

# Import target "fltk_z" for configuration "Release"
set_property(TARGET fltk_z APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_z PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk_z.lib"
  )

# Import target "fltk_jpeg" for configuration "Release"
set_property(TARGET fltk_jpeg APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_jpeg PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk_jpeg.lib"
  )

# Import target "fltk_png" for configuration "Release"
set_property(TARGET fltk_png APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_png PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk_png.lib"
  )

# Import target "fltk" for configuration "Release"
set_property(TARGET fltk APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C;CXX"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk.lib"
  )

# Import target "fltk_forms" for configuration "Release"
set_property(TARGET fltk_forms APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_forms PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk_forms.lib"
  )

# Import target "fltk_images" for configuration "Release"
set_property(TARGET fltk_images APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_images PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/proiect_2048/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk/lib/fltk_images.lib"
  )

# This file does not depend on other imported targets which have
# been exported from the same project but in a separate export set.

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
cmake_policy(POP)
