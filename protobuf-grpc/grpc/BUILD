load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_proto_grpc//java:defs.bzl", "java_grpc_library")
load("@rules_proto_grpc//java:defs.bzl", "java_proto_library")

java_proto_library(
    name = "server_java_proto",
    protos = [":server_proto"],
)

proto_library(
    name = "server_proto",
    srcs = ["server.proto"],
)

java_grpc_library(
    name = "server_java_grpc",
    protos = [":server_proto"],
)

java_binary(
    name = 'server',
    srcs = [
      'Server.java'
    ],
    deps = [
      ":server_java_proto",
      ":server_java_grpc",
    ],
    main_class = 'example.Server',
)

java_binary(
    name = 'client',
    srcs = [
      'Client.java'
    ],
    deps = [
      ":server_java_proto",
      ":server_java_grpc",
    ],
    main_class = 'example.Client',
)
