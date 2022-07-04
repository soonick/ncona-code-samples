# Introduction to Bazel - First binary

## Dependencies 

- Bazel: https://docs.bazel.build/versions/4.1.0/install.html
- JDK: https://www.oracle.com/java/technologies/downloads/

## Running example

Build:

```sh
bazel build //:example_package
```

Run from Bazel:

```sh
bazel run //:example_package
```

Run binary:

```sh
./bazel-bin/example_package
```
