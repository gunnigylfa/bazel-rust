# Bazel rust starter

This repository is intended to be a template for starting a rust project using the bazel build system.

## Project structure
Applications, i.e. things that will have a binary executable are contained within the  `applications/` directory.

Supporting libraries for these applications are contained with in the `libraries/` directory.

## How to build/run

You build the project and its target using as usually with bazel by running:

```
bazel build //... # builds the whole project
```

You can run the example application by running:
```
bazel run //applications/helloworld:app
```

## Dependency management
This project relies solely on bazel for dependency management and uses [`cargo-raze`](https://github.com/google/cargo-raze) to do so. In order to add a new dependency navigate to the 
[project Cargo.toml file](./Cargo.toml) and after adding the dependency there you will need to resolve the dependency file by running:
```
bazel run --legacy_external_runfiles @cargo_raze//:raze -- --manifest-path=$(realpath ./Cargo.toml)
```
After doing that the `Cargo.raze.lock`file should be updated and you able to use the new dependency.
### Using third party dependencies
Simply add it to the deps section of the `rust_library` or `rust_binary` like so:
```skylark
rust_binary(
    name = "app",
    srcs = ["src/main.rs"],
    deps = [
        "//libraries/greeter:greeter_lib",
        "//cargo:env_logger",
        "//cargo:log",
        "//cargo:rand",
    ]
)
```
## Editor Support

Currently it seems bazel and rust only play along well when using vscode and the `rust-analyser` plugin.
Before starting you will need to run 

```
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```
and every time you update a dependency. This command generates and updates the `rust-project.json` file used for autocompletion and goto definition for non-Cargo based projectss.