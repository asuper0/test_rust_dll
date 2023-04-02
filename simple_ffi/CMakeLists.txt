if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CMAKE_BUILD_TYPE "Release")
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()


set(RS_OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/${TARGET_DIR}")
set(RS_LIB_FILE "${RS_OUTPUT_DIR}")
set(CPP_BIN_PATH ..\\build\\cpp_demo\\${TARGET_DIR})

add_custom_target(simple_ffi ALL
    COMMENT "Compiling simple_ffi module"
    COMMAND cbindgen.exe --config .\\cbindgen.toml --crate simple_ffi --output ../cpp_demo/rust_api.h
    COMMAND ${CARGO_CMD}
    # Make sure that the cpp output path is exist
    COMMAND if not exist ${CPP_BIN_PATH} ( mkdir ${CPP_BIN_PATH})
    # only copy the dll if it's newer than the one exist in cpp output path
    COMMAND xcopy /D /Y target\\${TARGET_DIR}\\simple_ffi.dll ${CPP_BIN_PATH}\\

    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

set_target_properties(simple_ffi PROPERTIES LOCATION ${RS_LIB_FILE})

add_test(NAME rs_test
COMMAND cargo test
WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
