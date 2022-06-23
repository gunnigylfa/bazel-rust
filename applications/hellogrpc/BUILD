load("@rules_rust//proto:toolchain.bzl", "GRPC_COMPILE_DEPS")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "app",
    srcs = ["hello_server.rs"],
    visibility = ["//proto/hello:__subpackages__"],
    deps = [
        "//proto/hello:hello_rust_grpc",
    ] + GRPC_COMPILE_DEPS,
)