# optix-rs-pathtracer-example
![alt text](https://github.com/anderslanglands/optix-rs-pathtracer-example/blob/master/img/cornell_box_0001.png "Cornell Box")
Minimal example showing how to use optix-rs. This is a companion to https://github.com/anderslanglands/optix-rs - it is simply the pathtracer example from that crate factored out into its own repository to show how to set up the build for a project that depends on optix-rs.

# Dependencies
- optix-rs
- OptiX (tested on 5.0 and 5.1)
- CUDA (tested on 9.0-9.2)
- CMake (tested on 3.5 and above)

# Key things to note:
- the build.rs requires optix_root and cuda_root to be set either as environment variables or in a build-settings.toml file in order to be able to compile any ptx files and link to liboptix
- Add cuda programs to cuda/CMakeLists.txt to have them compiled by the build script. Edit cuda/ptx.cmake to tweak compile options.
- If you're on macOS, you'll need to add OPTIX_ROOT/lib64 to your DYLD_LIBRARY_PATH as Cargo doesn't currently support setting the rpath programatically.

