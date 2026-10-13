# 🦀 Rust → Wayland Shell
### Checklist belajar dari nol sampai punya desktop shell buatan sendiri

> *"Pahami dulu kenapa aturannya ada, baru bangun sesuatu di atasnya."*
> Roadmap ini untuk kamu yang sudah punya dasar **Go, C, Java**, dan ingin turun ke level sistem lewat **ricing Linux berbasis Wayland**.

---

## 🧭 Peta Perjalanan

| Fase | Fokus | Perkiraan | Hasil akhir |
|:---:|---|:---:|---|
| **1** | Inti bahasa & ownership | Minggu 1-6 | CLI + parser config |
| **2** | Memahami "kenapa" Rust | Minggu 7-12 | Daemon + Unix socket + hot reload |
| **3** | Rust bertemu Linux | Minggu 13-18 | Status provider daemon (JSON) |
| **4** | Fundamental Wayland | Minggu 19-24 | Panel layer-shell pertamamu |
| **5** | Shell sungguhan | Bulan 7-12 | Shell yang dipakai harian |
| **6** | Opsional: compositor | Bebas | Eksperimen dengan Smithay |

> ⏱️ Asumsi **8-10 jam per minggu**. Cepat atau lambat itu normal, yang penting gerbang kelulusan tiap fase terlewati.

### 📌 Cara pakai
- Centang `[x]` setiap item yang **benar-benar bisa kamu jelaskan atau lakukan tanpa menyontek**.
- Jangan lanjut ke fase berikutnya sebelum **🚦 Gerbang Kelulusan** lolos semua.
- Catat error yang bikin bingung di jurnal (template ada di bagian bawah).

### 📊 Progress Tracker
- [ ] 🌱 Fase 1 selesai
- [ ] 🌿 Fase 2 selesai
- [ ] 🔧 Fase 3 selesai
- [ ] 🪟 Fase 4 selesai
- [ ] 🚀 Fase 5 selesai
- [ ] 🧪 Fase 6 (opsional)

---

## 🛠️ Fase 0: Persiapan (1-2 hari)

- [x] Install Rust lewat **rustup**
- [x] Pastikan `rustc --version` dan `cargo --version` jalan
- [x] Install komponen: `rustup component add clippy rustfmt rust-analyzer`
- [x] Setup editor dengan **rust-analyzer** (VS Code, Neovim, Helix, atau Zed)
- [x] Jalankan `cargo new hello && cd hello && cargo run`
- [x] Biasakan `cargo clippy` dan `cargo fmt` **sejak hari pertama**

**📚 Sumber**

| Sumber | Link |
|---|---|
| Install Rust | https://www.rust-lang.org/tools/install |
| Rust Playground (coba kode di browser) | https://play.rust-lang.org/ |

---

## 🌱 Fase 1: Inti Bahasa (Minggu 1-6)

**🎯 Tujuan:** nyaman menulis program Rust biasa dan mengerti ownership dasar.

### ✅ Checklist konsep
- [x] Variabel, mutability (`let` vs `let mut`), shadowing
- [x] Tipe dasar, tuple, array, slice, `String` vs `&str`
- [ ] Fungsi, expression vs statement, control flow
- [ ] **Ownership & move**: kenapa `let b = a;` membuat `a` tidak bisa dipakai
- [ ] **Borrowing**: `&T` vs `&mut T` dan aturan "banyak pembaca *atau* satu penulis"
- [ ] `struct`, `impl`, method
- [ ] `enum`, pattern matching dengan `match`, `if let`
- [ ] `Option<T>` dan `Result<T, E>`
- [ ] Operator `?` dan error handling (`thiserror`, `anyhow`)
- [ ] `Vec`, `HashMap`, `HashSet`
- [ ] Iterator & closure (`map`, `filter`, `collect`)
- [ ] Module, crate, `Cargo.toml`, dependency

### 🧩 Mini-proyek
- [ ] **CLI pemilih wallpaper**: pilih gambar acak dari satu folder (`clap`)
- [ ] **Parser konfigurasi TOML** dengan `serde` + validasi dan pesan error yang jelas

### 🚦 Gerbang Kelulusan
- [ ] Bisa menjelaskan dengan kata sendiri kenapa `let b = a;` memindahkan kepemilikan
- [ ] Bisa memperbaiki error borrow checker **tanpa langsung `.clone()`**
- [ ] Bisa menulis fungsi yang mengembalikan `Result` dan menangani error-nya dengan `?`

### 📚 Sumber gratis

| Sumber | Untuk apa | Link |
|---|---|---|
| **The Rust Programming Language** | Bacaan utama, urut dari dasar | https://doc.rust-lang.org/book/ |
| **Rustlings** | Latihan kecil, langsung dikoreksi compiler | https://github.com/rust-lang/rustlings |
| **Rust by Example** | Belajar lewat contoh kode | https://doc.rust-lang.org/rust-by-example/ |
| **Comprehensive Rust** (Google) | Kursus padat, cocok untuk yang sudah bisa bahasa lain | https://google.github.io/comprehensive-rust/ |
| **Exercism, jalur Rust** | Latihan dengan feedback mentor sukarelawan | https://exercism.org/tracks/rust |
| **Command Line Apps in Rust** | Untuk mini-proyek CLI | https://rust-cli.github.io/book/ |
| **Serde** | Dokumentasi serialisasi | https://serde.rs/ |
| **Rust Cheat Sheet** | Rangkuman visual cepat | https://cheats.rs/ |

---

## 🌿 Fase 2: Memahami "Kenapa" Rust (Minggu 7-12)

**🎯 Tujuan:** paham alasan di balik aturan compiler, bukan sekadar menurutinya.

### ✅ Checklist konsep
- [ ] **Lifetime** pada fungsi (`fn longest<'a>(...)`)
- [ ] Lifetime pada `struct` yang menyimpan referensi
- [ ] `Box<T>`: kapan perlu alokasi heap
- [ ] `Rc<T>` dan `RefCell<T>` (interior mutability)
- [ ] `Arc<T>` dan `Mutex<T>`
- [ ] Trait: definisi, implementasi, default method
- [ ] Generics dan **trait bound**
- [ ] **Static dispatch (generics) vs dynamic dispatch (`dyn Trait`)**
- [ ] Thread (`std::thread`) dan channel (`mpsc`)
- [ ] Marker trait `Send` dan `Sync`, dan kenapa `Rc` tidak bisa dikirim antar-thread
- [ ] Trait `Drop` dan RAII
- [ ] Unit test dan integration test
- [ ] Pengenalan `unsafe` dan FFI (panggil fungsi C)

### 🧩 Mini-proyek
- [ ] **Daemon kecil** yang mendengarkan **Unix socket**, menerima perintah (misal `set-theme dark`), dan membalas
- [ ] Tambahkan **hot reload**: muat ulang config saat file berubah (crate `notify`)
- [ ] Pakai thread + channel dulu, **belum async**

### 🚦 Gerbang Kelulusan
- [ ] Bisa menjelaskan kapan memakai `Rc<RefCell<T>>` vs `Arc<Mutex<T>>`
- [ ] Bisa menjelaskan kenapa `Rc` tidak `Send`
- [ ] Bisa menjelaskan kapan `dyn Trait` lebih cocok daripada generics
- [ ] Bisa membaca error lifetime dan tahu apa yang compiler minta

### 📚 Sumber gratis

| Sumber | Untuk apa | Link |
|---|---|---|
| **The Book (bab 10, 15, 16)** | Generics/lifetime, smart pointer, konkurensi | https://doc.rust-lang.org/book/ |
| **Crust of Rust**, Jon Gjengset (YouTube) | Penjelasan mendalam lifetime, iterator, dll. | https://www.youtube.com/@jonhoo |
| **Learn Rust With Entirely Too Many Linked Lists** | Latihan ownership paling "menyiksa" tapi efektif | https://rust-unofficial.github.io/too-many-lists/ |
| **Rust Quiz** (dtolnay) | Uji pemahaman lewat soal jebakan | https://dtolnay.github.io/rust-quiz/ |
| **Rust Standard Library docs** | Referensi resmi | https://doc.rust-lang.org/std/ |
| **The Rustonomicon** | Bacaan lanjut tentang `unsafe` | https://doc.rust-lang.org/nomicon/ |
| **Rust API Guidelines** | Gaya penulisan API yang idiomatis | https://rust-lang.github.io/api-guidelines/ |
| **notify** (file watcher) | Dokumentasi crate | https://docs.rs/notify |

### 🧯 Kamus Error Borrow Checker
Saat error muncul, jalankan `rustc --explain KODE` untuk penjelasan resmi.

| Kode | Artinya | Biasanya karena |
|:---:|---|---|
| `E0382` | Nilai dipakai setelah dipindahkan | Lupa borrow atau clone |
| `E0499` | Dua `&mut` aktif sekaligus | Menahan referensi terlalu lama |
| `E0502` | `&` dan `&mut` bersamaan | Membaca sambil mengubah koleksi yang sama |
| `E0505` | Memindahkan nilai yang masih dipinjam | Referensi masih hidup |
| `E0597` | Nilai tidak hidup cukup lama | Mengembalikan referensi ke variabel lokal |
| `E0106` | Lifetime hilang | Fungsi/struct punya referensi tanpa anotasi |
| `E0277` | Trait bound tidak terpenuhi | Misalnya `Rc` dikirim ke thread |

---

## 🔧 Fase 3: Rust Bertemu Linux (Minggu 13-18)

**🎯 Tujuan:** bisa "bicara" dengan sistem: file virtual, D-Bus, dan banyak sumber event sekaligus.

### ✅ Checklist konsep
- [ ] Membaca data dari **`/proc`** dan **`/sys`** (baterai, brightness, suhu, CPU)
- [ ] Proses, sinyal, dan *exit code*
- [ ] **Event loop**: kenapa shell butuh satu loop untuk banyak sumber event
- [ ] Pengenalan `calloop` (event loop yang dipakai ekosistem Smithay)
- [ ] **D-Bus** dengan `zbus`: session bus vs system bus, method call, signal, property
- [ ] **Async/await** dasar: `Future` itu malas dan butuh executor
- [ ] Tokio: `spawn`, `select!`, channel async
- [ ] Kenapa `tokio::spawn` butuh `Send + 'static`
- [ ] Logging dengan `tracing`

### 🧩 Mini-proyek
- [ ] **Status provider daemon**: kumpulkan baterai, volume, jaringan, dan media yang diputar, lalu keluarkan sebagai **JSON** (stdout atau socket)
- [ ] Tes dengan **Waybar custom module** atau skrip shell biasa

### 🚦 Gerbang Kelulusan
- [ ] Bisa menggabungkan D-Bus + timer + socket dalam satu program tanpa data race
- [ ] Bisa menjelaskan kenapa data yang dikirim ke task async perlu dimiliki penuh atau dibungkus `Arc`
- [ ] Bisa membaca spesifikasi D-Bus freedesktop dan menerjemahkannya ke kode

### 📚 Sumber gratis

| Sumber | Untuk apa | Link |
|---|---|---|
| **Tokio Tutorial** | Async praktis, mulai dari sini | https://tokio.rs/tokio/tutorial |
| **Asynchronous Programming in Rust** | Konsep di balik `Future` dan `Pin` | https://rust-lang.github.io/async-book/ |
| **zbus Book** | D-Bus di Rust | https://dbus2.github.io/zbus/ |
| **calloop** | Event loop untuk ekosistem Wayland | https://github.com/Smithay/calloop |
| **Rust Cookbook** | Resep tugas umum (file, proses, dll.) | https://rust-lang-nursery.github.io/rust-cookbook/ |
| **Spesifikasi Notifikasi freedesktop** | Untuk notification daemon nanti | https://specifications.freedesktop.org/notification-spec/latest/ |
| **Spesifikasi MPRIS** | Kontrol pemutar media | https://specifications.freedesktop.org/mpris-spec/latest/ |
| **Spesifikasi Desktop Entry** | Untuk launcher (`.desktop`) | https://specifications.freedesktop.org/desktop-entry-spec/latest/ |

---

## 🪟 Fase 4: Fundamental Wayland (Minggu 19-24)

**🎯 Tujuan:** paham model Wayland sebelum memakai toolkit apa pun.

### ✅ Checklist konsep
- [ ] Beda **Wayland vs X11** secara arsitektur
- [ ] Peran **compositor** dan **client**
- [ ] **Global registry** dan *binding* ke interface
- [ ] `wl_surface`, `wl_output`, `wl_seat`, dan buffer
- [ ] Siklus hidup surface: configure, ack, commit
- [ ] **Protokol ekstensi**: kenapa banyak fitur shell tidak ada di protokol inti
- [ ] **`wlr-layer-shell`**: layer (background, bottom, top, overlay), anchor, exclusive zone
- [ ] **`ext-session-lock`**: dasar konsep lock screen
- [ ] Cek protokol yang didukung compositor kamu (`wayland-info`)

### 🧩 Mini-proyek
- [ ] **Panel polos** di tepi layar dengan `smithay-client-toolkit` lewat layer-shell
- [ ] Respon terhadap event output (monitor dicabut/dipasang)
- [ ] Hasilnya boleh jelek. Tujuannya paham siklus surface.

### 🚦 Gerbang Kelulusan
- [ ] Bisa menjelaskan apa itu layer-shell dan kenapa panel bisa "menempel" di tepi layar
- [ ] Bisa menjelaskan apa itu exclusive zone
- [ ] Bisa menjelaskan kenapa protokol tertentu ada di compositor A tapi tidak di B

### 📚 Sumber gratis

| Sumber | Untuk apa | Link |
|---|---|---|
| **The Wayland Protocol** (Drew DeVault) | Buku gratis, konsep dasar sampai protokol | https://wayland-book.com/ |
| **Wayland Explorer** | Jelajahi semua protokol dan versinya | https://wayland.app/protocols/ |
| **smithay-client-toolkit** | Library client Wayland di Rust | https://github.com/Smithay/client-toolkit |
| **ArchWiki: Wayland** | Praktik dan troubleshooting | https://wiki.archlinux.org/title/Wayland |
| **gtk4-layer-shell** | Layer-shell untuk GTK4 (ada binding Rust) | https://github.com/wmww/gtk4-layer-shell |

> ⚠️ Layer-shell **tidak didukung di GNOME-on-Wayland maupun desktop X11**. Shell buatanmu hanya akan jalan di compositor seperti Hyprland, Sway, niri, atau KDE Plasma.

---

## 🚀 Fase 5: Shell Sungguhan (Bulan 7-12)

**🎯 Tujuan:** punya shell yang kamu pakai sebagai setup harian.

### 🧰 Pilih toolkit

| Opsi | Kelebihan | Catatan |
|---|---|---|
| **GTK4 + gtk4-layer-shell + relm4** | Matang, widget lengkap, **bisa di-style dengan CSS** (sangat pas untuk ricing) | Butuh GTK |
| **iced + layer-shell** | Murni Rust, arsitektur Elm (update/view), enak dipelajari | Ekosistemnya lebih kecil |
| **Slint + layer-shika** | Sintaks UI deklaratif | Masih tahap awal, belum siap produksi |
| **smithay-client-toolkit mentah** | Paling banyak belajar | Paling banyak kerja manual |

> 💡 **Rekomendasi:** mulai dengan **GTK4 + relm4**, karena pengguna bisa mengubah tampilan lewat CSS tanpa kompilasi ulang. Ekosistem ini berubah cepat, jadi **cek status maintenance dan versi** tiap library sebelum memilih.

### ✅ Urutan komponen (dari termudah)

**1️⃣ Bar**
- [ ] Jam dan tanggal
- [ ] Workspace aktif (lewat IPC compositor)
- [ ] Baterai, volume, jaringan
- [ ] Multi-monitor

**2️⃣ Launcher**
- [ ] Baca daftar aplikasi dari file `.desktop`
- [ ] Pencarian fuzzy
- [ ] Jalankan aplikasi dan tutup launcher

**3️⃣ Notification daemon**
- [ ] Implementasi `org.freedesktop.Notifications` lewat `zbus`
- [ ] Tampilan popup + timeout + tombol aksi

**4️⃣ OSD (volume / brightness)**
- [ ] Overlay sementara saat nilai berubah

**5️⃣ System tray**
- [ ] Implementasi sisi *host* **StatusNotifierItem**

**6️⃣ Tema & konfigurasi**
- [ ] Konfigurasi TOML + **hot reload**
- [ ] Styling lewat CSS
- [ ] (Bonus) Generator palet warna dari wallpaper

**7️⃣ Lock screen** *(paling sensitif, taruh paling akhir)*
- [ ] Pakai `ext-session-lock`
- [ ] Pahami risiko keamanan sebelum dipakai sungguhan

### 🚦 Gerbang Kelulusan
- [ ] Kamu memakai shell buatanmu sendiri **minimal seminggu** sebagai setup harian
- [ ] Ada dokumentasi singkat dan screenshot di README
- [ ] Konfigurasi bisa diubah pengguna tanpa kompilasi ulang

### 📚 Sumber gratis

| Sumber | Untuk apa | Link |
|---|---|---|
| **GUI development with Rust and GTK 4** | Buku resmi gtk4-rs | https://gtk-rs.org/gtk4-rs/stable/latest/book/ |
| **Relm4 Book** | UI reaktif di atas GTK4 | https://relm4.org/book/stable/ |
| **iced** | Framework GUI Elm-style | https://github.com/iced-rs/iced |
| **Icebar** | Contoh status bar iced + layer-shell | https://github.com/RustyCoderX/Icebar |
| **margo** | Contoh arsitektur: compositor Smithay + shell GTK4/relm4 | https://github.com/kenanpelit/margo |
| **layer-shika** | Slint + layer-shell (eksperimental) | https://codeberg.org/waydeer/layer-shika |
| **hyprland-rs** | Binding IPC Hyprland | https://github.com/hyprland-community/hyprland-rs |
| **niri** | Compositor Rust, baca kodenya untuk belajar IPC | https://github.com/YaLTeR/niri |
| **Spesifikasi StatusNotifierItem** | Untuk system tray | https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/ |

> 🔎 Beberapa link bisa berubah. Kalau 404, cari nama resource-nya langsung di mesin pencari.

---

## 🧪 Fase 6 (Opsional): Compositor

- [ ] Baca kode compositor berbasis **Smithay** (niri, margo)
- [ ] Pahami DRM/KMS, input handling, dan rendering secara konsep
- [ ] Buat compositor minimal yang bisa menampilkan satu jendela

| Sumber | Link |
|---|---|
| **Smithay** | https://github.com/Smithay/smithay |
| **niri** (kode referensi) | https://github.com/YaLTeR/niri |

---

## 🧰 Lembar Contekan Crate

| Kebutuhan | Crate |
|---|---|
| Argumen CLI | `clap` |
| Serialisasi/config | `serde`, `toml` |
| Error handling | `thiserror`, `anyhow` |
| Pantau file | `notify` |
| Async runtime | `tokio` |
| D-Bus | `zbus` |
| Event loop Wayland | `calloop` |
| Client Wayland | `smithay-client-toolkit` |
| UI (GTK) | `gtk4`, `gtk4-layer-shell`, `relm4` |
| UI (Rust murni) | `iced` |
| Logging | `tracing` |
| IPC Hyprland | `hyprland` |

---

## 📓 Template Jurnal Mingguan

Salin tiap minggu. Menulis ulang masalahmu dengan kata sendiri adalah cara tercepat untuk paham.

```markdown
### Minggu ke-__

**Yang saya pelajari:**
-

**Error yang bikin bingung (kode + penyebab sebenarnya):**
-

**Konsep yang masih goyah:**
-

**Target minggu depan:**
- [ ]
```

---

## 🧠 Prinsip Selama Belajar

- 🚫 **Jangan lompat ke Fase 4-5 sebelum Fase 2 beres.** Kebanyakan orang menyerah karena langsung menghantam async + GUI + Wayland sekaligus.
- 🧱 Saat macet, **tulis versi paling sederhana dulu** (clone dan alokasi bebas), optimalkan belakangan.
- 📖 **Baca kode proyek nyata** sejak Fase 3, tidak perlu menunggu mahir.
- 🔍 Pakai `cargo clippy`. Saran-sarannya mengajarkan idiom Rust.
- 🧩 Saat mendesain struct, tanya: **"Siapa pemilik data ini?"**
- 🔁 Tulis ulang proyek lama setelah belajar konsep baru. Itu cara terbaik melihat kemajuanmu.

---

<div align="center">

**Selamat belajar! 🦀**
*Setiap error dari borrow checker adalah pelajaran gratis.*

</div>
