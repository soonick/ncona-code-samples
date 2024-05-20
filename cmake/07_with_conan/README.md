# An executable that uses dependencies from Conan

To build and run:

```
mkdir build
cd build
python3 -m venv ./venv
./venv/bin/pip3 install conan
./venv/bin/conan profile detect || true
./venv/bin/conan install .. --output-folder . --build missing
cmake .. -DCMAKE_TOOLCHAIN_FILE=conan_toolchain.cmake -DCMAKE_BUILD_TYPE=Release
make
./cmake-with-conan
```
