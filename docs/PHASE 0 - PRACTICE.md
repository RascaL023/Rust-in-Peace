# Installation Logs

```log
❯ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/rascal/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/rascal/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/rascal/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/rascal/.profile
  /home/rascal/.bashrc
  /home/rascal/.config/fish/conf.d/rustup.fish

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


    default host tuple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>

info: profile set to default
info: default host tuple is x86_64-unknown-linux-gnu
info: syncing channel updates for stable-x86_64-unknown-linux-gnu
info: latest update on 2026-10-01 for version 1.99.0 (b940084d7 2026-09-28)
info: downloading 6 components
cargo       installed                       10.45 MiB
clippy      installed                        4.99 MiB
rust-docs   installed                       23.18 MiB
rust-std    installed                       30.13 MiB
rustc       installed                       80.15 MiB
rustfmt     installed                        2.37 MiB
info: default toolchain set to stable-x86_64-unknown-linux-gnu

  stable-x86_64-unknown-linux-gnu installed - rustc 1.99.0 (b940084d7 2026-09-28)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source the
corresponding env file under $HOME/.cargo.

Consider running the right command for your shell (note the leading DOT):
. "$HOME/.cargo/env"           # For sh/ash/dash/pdksh/bash
source "$HOME/.cargo/env.fish" # For fish
cargo:rerun-if-env-changed=CC_x86_64-unknown-linux-gnu
CC_x86_64-unknown-linux-gnu = None
cargo:rerun-if-env-changed=CC_x86_64_unknown_linux_gnu
CC_x86_64_unknown_linux_gnu = None
cargo:rerun-if-env-changed=HOST_CC
HOST_CC = None
cargo:rerun-if-env-changed=CC
CC = None
cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
CRATE_CC_NO_DEFAULTS = None
cargo:rerun-if-env-changed=CFLAGS
CFLAGS = None
cargo:rerun-if-env-changed=HOST_CFLAGS
HOST_CFLAGS = None
cargo:rerun-if-env-changed=CFLAGS_x86_64_unknown_linux_gnu
CFLAGS_x86_64_unknown_linux_gnu = None
cargo:rerun-if-env-changed=CFLAGS_x86_64-unknown-linux-gnu
CFLAGS_x86_64-unknown-linux-gnu = None
```

# Verification

## Toolchain installation logs
```bash
# Toolchain Manager
~                                                                   
❯ rustup --version
rustup 1.29.1 (d95a37b6a 2026-08-13)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: the currently active `rustc` version is `rustc 1.99.0 (b940084d7 2026-09-28)`

# Compiler
~                                                                   
❯ rustc --version
rustc 1.99.0 (b940084d7 2026-09-28)

# Package Manager
~                                                                   
❯ cargo --version
cargo 1.99.0 (5f94df478 2026-08-27)
```

## After `cargo run` on target directory
```bash
phase-0 on  main [?]                                                                via  v1.99.0 
❯ tree target/
target/
├── CACHEDIR.TAG
├── debug
│   ├── build
│   ├── deps
│   │   ├── libphase_0-0b9d91b582c758a4.rmeta
│   │   ├── ...
│   │   ├── phase_0-0b9d91b582c758a4.d
│   │   ├── phase_0-6f7a37fdaef5577e
│   │   ├── phase_0-6f7a37fdaef5577e.d
│   │   ├── ...
│   ├── examples
│   ├── incremental
│   │   ├── phase_0-029m91fk7e5za
│   │   │   ├── s-hmwp65wtb5-1xd7yam-13cjyid17r41w0hukn5tfnpte
│   │   │   │   ├── 2c0z7t0lkebcn2z4tflrf0zrk.o
│   │   │   │   ├── ...
│   │   │   │   ├── dep-graph.bin
│   │   │   │   ├── query-cache.bin
│   │   │   │   └── work-products.bin
│   │   │   ├── s-hmwp65wtb5-1xd7yam.lock
│   │   │   ├── s-hmwp8mghdi-033n1rs-dmgaqzd39j87kqcdhsz5p8g1h
│   │   │   │   ├── 2c0z7t0lkebcn2z4tflrf0zrk.o
│   │   │   │   ├── ...
│   │   │   │   ├── dep-graph.bin
│   │   │   │   ├── query-cache.bin
│   │   │   │   └── work-products.bin
│   │   │   └── s-hmwp8mghdi-033n1rs.lock
│   │   ├── phase_0-0lf93fzfenw61
│   │   │   ├── s-hmwp642g91-1ob790d-59arf77dib7h10nlfygugt7l8
│   │   │   │   ├── dep-graph.bin
│   │   │   │   ├── query-cache.bin
│   │   │   │   └── work-products.bin
│   │   │   ├── s-hmwp642g91-1ob790d.lock
│   │   │   ├── s-hmwp8iwhj7-00lyux7-31fsrin1c43djt73ohlcu5tvl
│   │   │   │   ├── dep-graph.bin
│   │   │   │   ├── query-cache.bin
│   │   │   │   └── work-products.bin
│   │   │   └── s-hmwp8iwhj7-00lyux7.lock
│   │   ├── phase_0-0x1utrk3a6dsb
│   │   │   ├── s-hmwp95l499-0bdk66u-eymjehr2mn6udfjxxju4bbkhg
│   │   │   │   ├── dep-graph.bin
│   │   │   │   ├── query-cache.bin
│   │   │   │   └── work-products.bin
│   │   │   └── s-hmwp95l499-0bdk66u.lock
│   │   └── phase_0-2s6vg91psv3ps
│   │       ├── s-hmwp642eni-1u6nube-dcjczo8fzj3kiage33c1mldvw
│   │       │   ├── dep-graph.bin
│   │       │   ├── query-cache.bin
│   │       │   └── work-products.bin
│   │       ├── s-hmwp642eni-1u6nube.lock
│   │       ├── s-hmwp8iwgio-0tnemq7-87x57sg2rcvf1ttowku73r1bh
│   │       │   ├── dep-graph.bin
│   │       │   ├── query-cache.bin
│   │       │   └── work-products.bin
│   │       └── s-hmwp8iwgio-0tnemq7.lock
│   ├── phase-0                 <- Executable binary
│   └── phase-0.d               <- Dependency list
└── flycheck0
    ├── stderr
    └── stdout

18 directories, 53 files
```

## Explanations
- phase-0 dan phase-0.d: 
Ini adalah satu-satunya file yang sebenarnya Anda butuhkan untuk menjalankan program.
phase-0 adalah biner eksekutable, dan phase-0.d adalah file teks kecil yang berisi 
daftar dependensi (untuk sistem build mengetahui file mana yang perlu 
dikompilasi ulang jika ada yang berubah).

- deps/: 
Berisi artefak intermediate (perantara). 
Saat rustc mengompilasi kode, ia tidak langsung mengubahnya menjadi biner akhir 
dalam satu langkah. Ia menghasilkan file metadata (.rmeta) dan file objek (.o). 
File-file ini disimpan di sini agar bisa digunakan kembali.

- incremental/: 
Ini adalah jantung dari kecepatan kompilasi Rust. 
Rust menggunakan incremental compilation. Alih-alih mengompilasi ulang seluruh 
program dari nol setiap kali Anda menekan cargo run, rustc memecah kode menjadi 
bagian-bagian kecil, mengompilasinya, dan menyimpan hasilnya di dalam folder 
dengan nama hash yang acak (seperti s-hmwp65wtb5...). 
Jika Anda hanya mengubah satu baris di main.rs, rustc hanya akan mengompilasi ulang 
bagian yang berubah dan memuat sisanya dari cache incremental/ ini. 
Nama hash yang panjang itu adalah session hash dan crate hash untuk 
memastikan cache yang dimuat sesuai dengan versi kode yang tepat.

- flycheck0/: 
Ini adalah koneksi langsung dengan pertanyaan Anda sebelumnya tentang Neovim. 
Ketika Anda mengonfigurasi rust-analyzer, LSP tersebut berjalan di latar belakang 
untuk memberikan autocomplete dan error checking secara real-time saat Anda mengetik. 
Untuk melakukan ini tanpa mengunci folder target/debug/ (yang akan bentrok jika 
Anda secara bersamaan menjalankan cargo run di terminal), 
rust-analyzer menjalankan proses kompilasi terpisah di dalam folder flycheck0/.
