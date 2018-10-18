cmake_minimum_required(VERSION 3.5)
project(optix-rs)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_LIST_DIR})
find_package(OptiX)
find_package(CUDA)
include(ptx)

compile_optix_ptx(pathtracer.cu)
