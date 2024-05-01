# Minimal CMake example

To build and run:

```
mkdir -p build
cd build
cmake ..
make
./Hello
```

0r manually specifying the generator:

```
mkdir build
cd build
cmake -G "Unix Makefiles" ..
make
./Hello
```

Or build using CMake:

```
mkdir build
cd build
cmake -G "Unix Makefiles" ..
cmake --build . --target Hello
./Hello
```
