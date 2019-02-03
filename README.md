# maven_sidekick
A Rust crate to deal with large Maven projects. You give it an groupId and artifactID and it will parse all the projects and their dependencies.
Ideally in the future it will be able to help you do builds only on a subset of projects.
This is very much a work in progress.

Make sure you have clippy installed in order to build locally. Clippy is super nice for n00bs like me:
Get rust from https://rustup.rs/

```bash
rustup component add clippy-preview
```

I build it with:

```bash
cargo build && cargo test && cargo clippy
```

If you want to run it as a script, just install cargo script:
```bash
cargo install cargo-script
```

and execute it:

```bash
./mvnsk.rs
```

You can run the script tests like this:

```bash
cargo script --test mvnsk.rs
```