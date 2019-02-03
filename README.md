# maven_sidekick
A Rust crate to deal with large Maven projects

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