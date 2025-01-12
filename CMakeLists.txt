cmake_minimum_required(VERSION 3.19)
project(rust-talk-demo LANGUAGES CXX)

# See https://github.com/rust-lang/rust/issues/39016
if(MSVC)
    list(REMOVE_ITEM CMAKE_CONFIGURATION_TYPES "Debug")
endif()

# Get Corrosion
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5.1
)
FetchContent_MakeAvailable(Corrosion)

# Use Corrsion to import crates from Cargo as CMake targets
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

# Use Corrsion cxx integration to generate the C++ side of the
# FFI and import it to CMake as a static library target. Automatically
# downloads the `cxxbridge` CLI.
corrosion_add_cxxbridge(bindings_ffi
    CRATE bindings
    FILES lib.rs
)

target_include_directories(bindings_ffi PUBLIC ${CMAKE_SOURCE_DIR})

# Create the final executable with the C++ glue
add_executable(magiclib_cpp
    main.cpp
)

target_link_libraries(magiclib_cpp PRIVATE bindings bindings_ffi)
