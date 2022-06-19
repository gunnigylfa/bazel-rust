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
This project relies solely on bazel for dependency management and uses [`crate_universe`](https://bazelbuild.github.io/rules_rust/crate_universe.html) to do so. In order to add a new dependency navigate to the 
[project WORKSPACE file](./WORKSPACE) and after adding the dependency there you will need to re-render  the lock file by running: 
``` 
CARGO_BAZEL_REPIN=1 bazel sync --only=crates
```

## Editor Support

Currently it seems bazel and rust only play along well when using vscode and the `rust-analyser` plugin.
Before starting you will need to run 

```
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```
and every time you update a dependency. This command generates and updates the `rust-project.json` file used for autocompletion and goto definition for non-Cargo based projectss.