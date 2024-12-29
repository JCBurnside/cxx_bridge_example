cmake_minimum_required(VERSION 3.31)
set(CMAKE_CXX_STANDARD 17)
project(FooDialect)
# find_program(CXXBRIDGE_FOUND "cxxbridge")
# set(CXXBRIDGE "-notfound")
# if(CXXBRIDGE_FOUND)
# else()
#     add_custom_command(
#         OUTPUT CXXBRIDGE
#         COMMAND cargo install cxxbridge
#     )
#     set(CXXBRIDGE "cxxbridge")
# endif()
# find_package(LLVM CONFIG REQUIRED 18)
# include(AddLLVM)
include_directories(include)
# set(BRDIGE_CXX "${CMAKE_CURRENT_BINARY_DIR}/dialect_lib.cpp")
# add_library(bridge STATIC ${BRDIGE_CXX})
# add_custom_command(
#     OUTPUT ${BRDIGE_CXX}
#     COMMAND ${CXXBRIDGE} ../dialect_lib/src/lib.rs > ${BRIDGE_CXX}
#     WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
# )
add_library(foo_dialect STATIC src/lib.cpp)
# target_link_libraries(foo_dialect PUBLIC bridge)
install(
    TARGETS foo_dialect
)