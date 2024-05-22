# An executable that uses dependencies from Vcpkg

Make sure vcpkg is installed and `VCPKG_ROOT` env variable is set.

To build and run:

```
mkdir build
cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake
make
./cmake-with-vcpkg
```
