#
# UseFLTK.CMake - FLTK CMake environment configuration file for external projects.
#
# This file is deprecated and will be removed in FLTK 1.4.0 or later
#
# automatically generated - do not edit
#

include_directories("C:/Users/Daniel/Desktop/Rust 2023/flkt_ex/target/debug/build/fltk-sys-4f377a44d60ae188/out/build/fltk;C:/Users/Daniel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.19/cfltk/fltk")

message(AUTHOR_WARNING
" * Warning: UseFLTK.cmake is deprecated and will be removed in FLTK 1.4.0
 * or later. Please use 'include_directories(\${FLTK_INCLUDE_DIRS})' or
 * 'target_include_directories(<target> PUBLIC|PRIVATE \${FLTK_INCLUDE_DIRS})'
 * instead of 'include(\${FLTK_USE_FILE})'.")
