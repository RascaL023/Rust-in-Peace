# PREPARATION

## Environments
Apa saja environtment dalam Rust?
Secara garis besar:
```text
                    PROJECT RUST
                         │
                         ▼
                      Cargo
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
            rustc              Dependency
         compiler              dari crates.io
              │
              ▼
       Rust standard library
              │
              ▼
          Toolchain
              │
              ▼
            rustup
              │
              ▼
        OS / Windows / Linux

```

---
Perbandingan environment:

| Nama            | Apa?                               | Analogi sederhana                              |
| --------------- | ---------------------------------- | ---------------------------------------------- |
| `rustup`        | Pengelola instalasi/toolchain Rust | `nvm`, `pyenv`, `rbenv`                        |
| `rustc`         | Compiler Rust                      | `gcc`, `clang`, `javac`                        |
| `cargo`         | Build system + package manager     | campuran `go`, `go build`, `npm`, Maven/Gradle |
| `rust-analyzer` | Language server untuk editor       | IntelliSense + analisis kode                   |
| `rustfmt`       | Formatter                          | `gofmt`, `clang-format`                        |
| `clippy`        | Linter                             | static analysis / lint                         |


### rustup
rustup adalah **toolchain manager**, yang mana dapat memasang beberapa toolchain Rust dan menentukkan mana yang digunakkan.
Toolchain sendiri dapat dikatakkan sebagai kotak/paket yang berisikan alat/environment dalam membantu coding. 
Bentuknya adalah file binary, contohnya `stable-x86_64-unknown-linux-gnu`.
Di dalamnya bisa berisikan compiler, paket manager, dan lainya. rustup sendiri adalah manager dari toolchain tersebut.

rustup adalah program baris perintah (CLI) yang bertugas:

> Mengunduh dan menginstal toolchain Rust ke komputer Anda.
> Mengelola banyak toolchain secara bersamaan. Rust memiliki tiga "saluran" (channel) rilis utama:
> - stable: Versi resmi yang stabil (dirilis setiap 6 minggu). Ini yang akan paling sering Anda pakai.
> - beta: Versi uji coba sebelum menjadi stable.
> - nightly: Versi eksperimen yang diperbarui setiap malam, berisi fitur-fitur paling baru tapi belum stabil.
>
> Memungkinkan Anda berpindah antar toolchain dengan mudah. 
Misalnya, Anda bisa menginstal toolchain stable untuk proyek A, 
toolchain nightly untuk proyek B, tanpa mereka saling bertabrakan.

Ketika Anda menjalankan perintah `rustup default stable`, Anda sedang memberi tahu rustup: 
> *"Tolong siapkan toolchain versi stable, 
> jadikan ini sebagai alat default setiap kali 
> saya mengetik cargo atau rustc di terminal."*

Secara mental model:
```text
rustup
  │
  ├── stable
  │     ├── rustc
  │     ├── cargo
  │     └── standard library
  │
  ├── beta
  │     ├── rustc
  │     └── ...
  │
  └── nightly
        ├── rustc
        └── ...
```

### rustc
rustc adalah **compiler** dari Rust. 
Sederhananya, compiler adalah penerjemah dari kode pemrograman (dalam kasus ini adalah Rust), 
menjadi executable atau kode yang dapat di eksekusi oleh mesin. Contohnya: `rustc main.rs`, yang mana akan menghasilkan executable.
Namun pada praktik modern nya, kita akan lebih sering pakai `cargo build`. Sebab cargo menangani lebih banyak hal.

### cargo
cargo sendiri adalah package manager dan build tool dari Rust. Bisa disamakan dengan npm pada ekosistem nodejs.
Contoh perintah cargo:
> `cargo new hello`     =>  membuat project
> `cargo build`         =>  compile project
> `cargo run`           =>  compile dan run project
> `cargo check`         =>  periksa kode apakah bisa di compile tanpa menghasilkan executable
> `cargo test`          =>  menjalankan test
> `cargo fmt`           =>  memformat kode
> `cargo clippy`        =>  menjalankan clippy

Salah satu hal utama yang di urus oleh cargo adalah dependency. 
Nantinya akan ada Cargo.toml yang berisikan manifest project, contohnya:
```toml
[dependencies]
toml = "..."
serde = "..."
```
Bisa juga terdapat seperti metadata project, seperti name, description, dsb.
Padanan nya dengan nodejs atau npm adalah `package.json` = `Cargo.toml`, `package-lock.json` = `Cargo.lock`

### rust-analyzer
rust-analyzer adalah LSP untuk Rust.
Apa itu LSP? Sederhananya ia adalah Language Server yang membantu kita dalam coding.
Contohnya saat kita buka IDE, VSCode misalnya, kemudian kita mengetikkan:
```rust
le..
```
Biasanya akan muncul semacam suggestion/autocomplete. Itu adalah hasil kerja LSP.
Kemudian IDE bisa:
- memberi autocomplete
- menunjukkan error
- melakukan jump-to-definition
- menunjukkan tipe
- rename symbol
- memberikan informasi function
- membantu navigasi project

### rustfmt
Singkat saja, rustfmt adalah formatter. Bisa dikatakkan sebagai prettier.
Misal kita menulis secara agak aneh tapi tetap bisa jalan:
```rust
fn main(){println!("Hello");}
```
Dengan menuliskan `cargo fmt`, maka kode tersebut akan menjadi:
```rust
fn main() {
    println!("Hello");
}
```

### clippy
clippy adalah salah satu tool yang cukup berguna ketika belajar.
Ia mencari pola kode yang mungkin:
- kurang idiomatis
- tidak diperlukan
- rawan kesalahan
- bisa dibuat lebih sederhana

Jika compiler berkata:
> "Kode ini valid".

Maka clippy bisa berkata:
> "Valid sih, tapi Rust biasanya ditulis dengan cara yang lebih baik seperti ini."

---
## Conclution Map

```text
                 Environment Rust
                        │
       ┌────────────────┼────────────────┐
       ▼                ▼                ▼
     rustup            Cargo          Editor
       │                │                │
   toolchain         project       rust-analyzer
       │
   ┌───┴────┐
   ▼        ▼
 stable    components
            │
       ┌────┼────┐
       ▼    ▼    ▼
   rustfmt clippy analyzer
```

```text
┌──────────────────────────────────┐
│          Rust Development        │
│                                  │
│  Editor                          │
│    │                             │
│    └── rust-analyzer             │
│                                  │
│  Cargo                           │
│    ├── build                     │
│    ├── run                       │
│    ├── check                     │
│    ├── test                      │
│    ├── fmt ──► rustfmt           │
│    └── clippy ─► Clippy          │
│                                  │
│  rustup                          │
│    │                             │
│    └── stable toolchain          │
│           ├── rustc              │
│           ├── cargo              │
│           └── std                │
└──────────────────────────────────┘
```
