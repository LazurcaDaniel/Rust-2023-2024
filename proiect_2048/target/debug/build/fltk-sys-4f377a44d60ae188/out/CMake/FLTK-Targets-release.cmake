#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "fltk_z" for configuration "Release"
set_property(TARGET fltk_z APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_z PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/fltk_z.lib"
  )

list(APPEND _cmake_import_check_targets fltk_z )
list(APPEND _cmake_import_check_files_for_fltk_z "${_IMPORT_PREFIX}/lib/fltk_z.lib" )

# Import target "fltk_jpeg" for configuration "Release"
set_property(TARGET fltk_jpeg APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_jpeg PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/fltk_jpeg.lib"
  )

list(APPEND _cmake_import_check_targets fltk_jpeg )
list(APPEND _cmake_import_check_files_for_fltk_jpeg "${_IMPORT_PREFIX}/lib/fltk_jpeg.lib" )

# Import target "fltk_png" for configuration "Release"
set_property(TARGET fltk_png APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_png PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/fltk_png.lib"
  )

list(APPEND _cmake_import_check_targets fltk_png )
list(APPEND _cmake_import_check_files_for_fltk_png "${_IMPORT_PREFIX}/lib/fltk_png.lib" )

# Import target "fltk" for configuration "Release"
set_property(TARGET fltk APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "C;CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/fltk.lib"
  )

list(APPEND _cmake_import_check_targets fltk )
list(APPEND _cmake_import_check_files_for_fltk "${_IMPORT_PREFIX}/lib/fltk.lib" )

# Import target "fltk_forms" for configuration "Release"
set_property(TARGET fltk_forms APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_forms PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/fltk_forms.lib"
  )

list(APPEND _cmake_import_check_targets fltk_forms )
list(APPEND _cmake_import_check_files_for_fltk_forms "${_IMPORT_PREFIX}/lib/fltk_forms.lib" )

# Import target "fltk_images" for configuration "Release"
set_property(TARGET fltk_images APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(fltk_images PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/fltk_images.lib"
  )

list(APPEND _cmake_import_check_targets fltk_images )
list(APPEND _cmake_import_check_files_for_fltk_images "${_IMPORT_PREFIX}/lib/fltk_images.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
