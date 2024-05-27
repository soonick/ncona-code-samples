# An executable that uses dependencies from Conan

Before:

- Install Arduino CLI
- Install `arduino.renesas_uno` board

To build:

```
mkdir build
cp arduino build/
cd build
git clone https://github.com/technyon/Arduino-CMake-Toolchain.git
cd Arduino-CMake-Toolchain
git checkout 3c624d2e5f8745251fddb1526de068bab56d7c0e
cd ..
python3 -m venv ./venv
./venv/bin/pip3 install conan
./venv/bin/conan profile detect || true
./venv/bin/conan install .. --output-folder . --build missing --profile=arduino
cmake .. -DCMAKE_TOOLCHAIN_FILE=conan_toolchain.cmake -DCMAKE_BUILD_TYPE=Release -D ARDUINO_INSTALL_PATH=$HOME/.arduino15 -D ARDUINO_ENABLE_PACKAGE_MANAGER=ON -D ARDUINO_PLATFORM=arduino.renesas_uno -D ARDUINO_BOARD_OPTIONS_FILE=BoardOptions.cmake -D CONAN_CXX_FLAGS="" -D CONAN_C_FLAGS="" -D CONAN_SHARED_LINKER_FLAGS="" -D CONAN_EXE_LINKER_FLAGS="" -DCMAKE_CXX_COMPILER_FORCED=TRUE
make
```
