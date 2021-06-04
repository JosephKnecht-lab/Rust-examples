# Rust-examples

Basic examples in Rust Programming language

For learning purposes only

# Commands

* Create new project 

```
cargo new <project name>
```

+ Build project: 
```
cargo build <project name>
```
Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile Cargo uses when you run cargo build --release.

+ Run project: 
```
cargo run <project name>
```

+ Test project: 
```
cargo test
```

+ Create  documentation: 
```
cargo doc
```
To open documentation use cargo doc --open

+ Install Cargo.io packages: 
```
cargo install <name of package>
```
All binaries installed with cargo install are stored in the installation root’s bin folder at $HOME/.cargo/bin.
If the project has no binaries, it has to be added as dependany at Cargo.toml