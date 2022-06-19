load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rustfmt_test"
)

rust_library(
    name = "greeter_lib",
    srcs = [
        "src/greeter.rs",
        "src/lib.rs",
    ],
    crate_features = ["default"],
    rustc_flags = ["--cap-lints=allow"],
)

package(default_visibility = ["//visibility:public"])


rustfmt_test(
    name = "fmt-check",
    targets = [":greeter_lib"]
)


