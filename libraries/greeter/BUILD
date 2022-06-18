load(
    "@rules_rust//rust:defs.bzl",
    "rust_doc",
    "rust_doc_test",
    "rust_library",
    "rust_shared_library",
    "rust_static_library",
    "rust_test",
)

rust_library(
    name = "greeter_lib",
    srcs = [
        "src/greeter.rs",
        "src/lib.rs",
    ],
    crate_features = ["default"],
    edition = "2018",
    rustc_flags = ["--cap-lints=allow"],
)

package(default_visibility = ["//visibility:public"])


