# Passing Arguemnts To A Java Binary Ran With Bazel

## Run java example passing a JVM flag

Build:

```sh
bazel run //:main --jvmopt="-Xdebug"
```

## Use jcmd to verify the flag was passed correctly

```bash
jcmd $PID VM.command_line

316721:
VM Arguments:
jvm_args: -Xdebug
java_command: example.Main
java_class_path (initial): main.jar
Launcher Type: SUN_STANDARD
```
