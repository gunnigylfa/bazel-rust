load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_rust//proto:proto.bzl", "rust_grpc_library")

proto_library(
    name = "hello",
    srcs = ["helloworld.proto"],
    visibility = ["//visibility:public"],
)

rust_grpc_library(
    name = "hello_rust_grpc",
    visibility = ["//visibility:public"],
    deps = [":hello"],
)
