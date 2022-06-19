load("@rules_rust//rust:defs.bzl", "rust_binary", "rustfmt_test")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "app",
    srcs = ["src/main.rs"],
    deps = ["//libraries/greeter:greeter_lib"],
)

rustfmt_test(
    name = "fmt-check",
    targets = [":app"]
)
