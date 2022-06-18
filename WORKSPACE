workspace(name = "rustmono")


load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_rust",
    sha256 = "872b04538ca20dad94791c348623f079ba93daf274c1d57ae6bfe0930ec77f0d",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.6.0/rules_rust-v0.6.0.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.6.0/rules_rust-v0.6.0.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(
    edition = "2018",
    include_rustc_srcs = True,
    version = "1.54.0",
)

# Rust analyzer
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_deps")

rust_analyzer_deps()

# Crate universe dependencies without Cargo
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate", "render_config")

crate_universe_dependencies()

crates_repository(
    name = "crates",
    packages = {
            "tokio": crate.spec(
                version = "1.12.0",
            ),
        },
    # REPIN=1 bazel build ...
    lockfile = "//:Cargo.Bazel.lock",
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crates//:defs.bzl", "crate_repositories")

crate_repositories()
