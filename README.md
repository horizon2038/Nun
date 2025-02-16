# Nun

Nun is a Runtime for creating OS to use A9N Microkernel. 
"Nun" is named after the primordial god in Egptian mythology.

## Author

horizon2k38 ( Rekka "horizon" IGUMI )

## Build Your Own OS with Nun

### 1. Create a new project
```bash
mkdir your-project
cd your-project
cargo new --bin your-os
```

### 2. Clone this repository

You can choose to either clone the repository directly or add it as a Git submodule.

#### Clone Directly

```bash
git clone https://github.com/horizon2038/Nun.git
```

#### Add as a Git Submodule

> [!NOTE]
> TODO

### 3. Add Nun to your project as a workspace

#### make `Cargo.toml` in your project root

```bash
touch Cargo.toml
```

#### Edit `Cargo.toml`
```toml
[package]
name = "your-project"
edition = "2021"

[workspace]
members = [
    "your-os",
    "Nun",
]

[dependencies]
nun = { path = "Nun" }

[[bin]]
name = "your-os"
path = "your-os/src/main.rs"

```

### 4. Copy `./.cargo/config.toml` to your project root

```bash
mkdir .cargo/
cp -r Nun/.cargo/config.toml ./.cargo/
```

### 5. Write your OS

#### Code Example - `your-os/src/main.rs`

```rust
#![no_std]
#![no_main]

// using Nun!
use nun;

// configure entry point
nun::entry!(main);

fn main(init_info: &nun::InitInfo) {
    nun::println!("Hello, Nun World!");

    loop {}
}
```

###  6. Build

```bash
cargo build --target Nun/arch/{arch}-unknown-a9n.json -Z build-std-features=compiler-builtins-mem --release

```

## Author

Email : rekka728 "at" gmail.com  
X : [@horizon2k38](https://x.com/horizon2k38)  

## Acknowledgements

[MITOU IT](https://www.ipa.go.jp/jinzai/mitou/it/2024/gaiyou-sg-2.html) : This project was supported by the MITOU IT program.

## License

[MIT License](https://choosealicense.com/licenses/mit/)
