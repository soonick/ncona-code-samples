# Introduction to cadence workflows

## Server

Go to the `server` folder and run:

```bash
docker-compose up
```

Keep the server running in a terminal window.

Register a domain for the workflows:

```bash
docker run --network=host --rm ubercadence/cli:master --do test-domain domain register -rd 1
```

## Hello world

This is the hello world example from Cadence documentation: https://cadenceworkflow.io/docs/get-started/java-hello-world/, but this one actually works.

To run, go to `hello-world/` and run:

```bash
bazel run :hello_world
```

Keep the worker running in a terminal window and execute the following command in another terminal:

```bash
docker run --network=host --rm ubercadence/cli:master \
    --do test-domain \
    workflow start \
    --tasklist HelloWorldList \
    --workflow_type HelloWorld::sayHello \
    --execution_timeout 3600 \
    --input \"Jose\"
```

You will see the message `Hello Jose!` printed in the worker terminal.
