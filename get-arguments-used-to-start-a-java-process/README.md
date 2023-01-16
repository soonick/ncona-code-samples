# Get arguments used to start a java process

## Run java example

Build:

```sh
bazel run //:main --jvmopt="-Xdebug"
```

## Using ps

```bash
ps -f $PID

UID          PID    PPID  C STIME TTY      STAT   TIME CMD
adrian    316721  313854  0 16:15 pts/6    Sl+    0:00 /home/adrian/.cache/bazel/_bazel_adrian/28381a26654a75034a8803698f5ef496/execroot/__main__/bazel-out/k8-fastbuild/bin/main.runfiles/local_jdk/bin/java -classpath main.jar -Xdebug example.Main
```

## Using jps

```bash
jps -lvm
```

## Using jcmd

```bash
jcmd $PID VM.command_line

316721:
VM Arguments:
jvm_args: -Xdebug
java_command: example.Main
java_class_path (initial): main.jar
Launcher Type: SUN_STANDARD
```

`jcmd` can also be used to get more information about the JVM configuration for a process:

```bash
jcmd $PID VM.flags
jcmd $PID VM.system_properties
```
