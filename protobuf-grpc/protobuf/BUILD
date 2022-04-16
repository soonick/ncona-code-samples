load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_proto_grpc//java:defs.bzl", "java_proto_library")

java_proto_library(
    name = "person_java_proto",
    protos = [":person_proto"],
)

proto_library(
    name = "person_proto",
    srcs = ["person.proto"],
)

java_binary(
    name = 'main',
    srcs = [
      'Main.java'
    ],
    deps = [":person_java_proto"],
    main_class = 'example.Main',
)
