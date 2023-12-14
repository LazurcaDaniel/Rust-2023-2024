#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "cfltk::cfltk" for configuration "Release"
set_property(TARGET cfltk::cfltk APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(cfltk::cfltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "C:/Users/Daniel/Desktop/Rust 2023/flkt_ex/target/debug/build/fltk-sys-4f377a44d60ae188/out/lib/cfltk.lib"
  )

list(APPEND _cmake_import_check_targets cfltk::cfltk )
list(APPEND _cmake_import_check_files_for_cfltk::cfltk "C:/Users/Daniel/Desktop/Rust 2023/flkt_ex/target/debug/build/fltk-sys-4f377a44d60ae188/out/lib/cfltk.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
