# Catatan: Cara Menghadapi Istilah Asing

> **Masalah:** saat belajar fitur A, tiba-tiba muncul sintaks/istilah yang belum pernah dibahas
> (`From`, `#[derive(...)]`, `dyn`, `'a`, `trait`, dll.), lalu fokus buyar.
> **Penyebab:** fitur Rust saling terkait, jadi dokumentasi dan AI sering menyebut fitur lain tanpa menjelaskan.
> **Solusi:** jangan coba paham semuanya. Kelompokkan, lalu putuskan cepat.

---

## Tiga Kategori

| Tanda | Artinya | Tindakan |
|:---:|---|---|
| [SKIP] | **Kotak hitam** | Pakai saja. Cukup tahu efeknya, abaikan cara kerjanya |
| [NEXT] | **Parkir** | Catat di log, pelajari di fase yang disebutkan |
| [STOP] | **Blokir** | Berhenti 10-15 menit dan pelajari sekarang, karena tanpa ini kode tidak terbaca |

**Pertanyaan penentu:**
> *"Kalau saya abaikan ini, apakah baris di sekitarnya jadi tidak bisa dipahami?"*
> Tetap bisa dipahami → [SKIP] atau [NEXT]. Tidak bisa → [STOP].

**Aturan tambahan**
- Tanda `!` di belakang nama (`println!`, `vec!`, `anyhow!`) = **macro**. Pakai saja [SKIP].
- Tanda `#[...]` = **attribute** (catatan untuk compiler). Pakai saja [SKIP].
- Tanda `<...>` = **generics**. Baca sebagai "berisi/bertipe ...". Contoh `Vec<String>` = "Vec yang berisi String" [SKIP].
- Tanda `'a` atau `'static` = **lifetime**. [NEXT] sampai Fase 2.
- Kalimat "X mengimplementasi trait Y" = "tipe X punya kemampuan Y".

---

## Prompt Pembuka untuk AI

> Saya baru belajar sampai **[topik]**. Jelaskan hanya dengan konsep yang sudah saya pelajari.
> Kalau butuh konsep lain, sebutkan namanya dan beri **satu kalimat saja**, jangan dijelaskan panjang.
> Tandai bagian mana yang "pakai dulu, dipahami nanti".

---

## FASE 1: Peta Istilah per Topik

### Variabel, mutability, tipe dasar
- [ ] [SKIP] `println!`, `format!` (macro)
- [ ] [SKIP] `{}` dan `{:?}` di string format (`{:?}` = tampilan debug)
- [ ] [SKIP] `usize`, `u8`, `i32`, `as` (konversi angka)
- [ ] [SKIP] `Vec<T>`, `<T>` dibaca "berisi T"
- [ ] [NEXT] `const`, `static` (Fase 2)
- [ ] [NEXT] `&'static str` (tipe literal string; terkait lifetime, Fase 2)

### Fungsi, control flow
- [ ] [SKIP] `->` (tipe return), `()` (unit, artinya "tidak ada nilai")
- [ ] [NEXT] `impl Trait` di parameter (Fase 2)
- [ ] [NEXT] `!` sebagai tipe return / `never` (jarang, abaikan)

### Ownership & borrowing
- [ ] [STOP] `Copy` vs move: angka/bool/char disalin otomatis, `String`/`Vec` dipindahkan
- [ ] [SKIP] `*x` (dereference, mengambil nilai di balik referensi)
- [ ] [SKIP] `&&x` di closure `filter`: tambah/kurangi `&` dan `*` sampai compile
- [ ] [NEXT] `'a` (lifetime) → Fase 2
- [ ] [NEXT] `Drop` → Fase 2
- [ ] [NEXT] `ref`, `ref mut` di pattern (jarang)

### struct, impl, method
- [ ] [STOP] `self`, `&self`, `&mut self`, `Self`
- [ ] [SKIP] `Type::new(...)` (fungsi asosiasi, mirip static method)
- [ ] [SKIP] `pub`, `mod`, `use`
- [ ] [SKIP] `#[derive(Debug, Clone, Default)]` ("otomatiskan kemampuan ini")

### enum, match
- [ ] [SKIP] `_` (tangkap semua sisanya), `if` guard di `match`
- [ ] [SKIP] `matches!(x, Pola)` (macro)
- [ ] [NEXT] `Box<T>` untuk enum rekursif → Fase 2
- [ ] [NEXT] `@` binding di pattern (jarang)

### Option & Result
- [ ] [SKIP] `unwrap()`, `expect("...")` (hanya untuk belajar/prototipe, bukan kode serius)
- [ ] [SKIP] `map`, `ok_or`, `unwrap_or`, `unwrap_or_default`
- [ ] [SKIP] `|x| ...` (closure, mirip lambda)
- [ ] [NEXT] `Box<dyn Error>` → Fase 2

### 🆕 Trait dasar *(ditambahkan sebelum error handling)*
- [ ] [STOP] Apa itu trait: kumpulan kemampuan, setara `interface` di Go/Java
- [ ] [STOP] `impl Trait for Tipe` dan `trait Nama { fn ...; }`
- [ ] [STOP] `#[derive(...)]` = minta compiler menulis implementasinya
- [ ] [STOP] Trait bawaan yang sering muncul: `Debug`, `Display`, `Clone`, `From`
- [ ] [SKIP] "Method milik trait baru bisa dipanggil kalau trait-nya di-`use`"
- [ ] [NEXT] `T: Trait` (trait bound), `where`, `dyn Trait` → Fase 2
- [ ] [NEXT] `type Item` / associated type → Fase 2

### Operator `?` dan error handling
- [ ] [STOP] `?` = "kalau `Err`, return langsung; kalau `Ok`, ambil isinya", dan **konversi error otomatis lewat `From`**
- [ ] [STOP] `From<A> for B` = "B bisa dibuat dari A"
- [ ] [SKIP] `std::error::Error`, `Display` (syarat tipe error yang layak)
- [ ] [SKIP] `#[derive(thiserror::Error)]`, `#[error("...")]`, `#[from]`
- [ ] [SKIP] `use anyhow::{Context, Result}`: `Result` hanya alias pemendek, `Context` trait yang memberi `.context("...")`
- [ ] [SKIP] `anyhow!`, `bail!` (macro pembuat error cepat)
- [ ] [SKIP] `fn main() -> anyhow::Result<()>`
- [ ] [NEXT] `Box<dyn Error + Send + Sync + 'static>` → Fase 2

### Collection, iterator, closure
- [ ] [STOP] `.iter()` vs `.iter_mut()` vs `.into_iter()`: meminjam, meminjam untuk diubah, atau mengambil alih
- [ ] [SKIP] `collect::<Vec<_>>()` (`::<>` disebut turbofish, `_` artinya "compiler tebak")
- [ ] [SKIP] `.entry(k).or_insert(v)` pada `HashMap`
- [ ] [SKIP] Tipe key `HashMap` butuh `#[derive(Hash, Eq, PartialEq)]`
- [ ] [NEXT] `Fn`, `FnMut`, `FnOnce`, `impl Fn(...)` → Fase 2
- [ ] [NEXT] `move || ...` → Fase 2 (muncul bersama thread)
- [ ] [NEXT] trait `Iterator` yang ditulis sendiri → Fase 2

### Module, crate, Cargo
- [ ] [SKIP] `crate::`, `super::`, `pub(crate)`
- [ ] [SKIP] `features = ["derive"]` di `Cargo.toml`
- [ ] [SKIP] `#[cfg(test)]`, `#[test]`
- [ ] [NEXT] workspace (banyak crate dalam satu repo)

---

## FASE 2: Istilah yang Akan Muncul

- [ ] [STOP] Lifetime `'a`: ini fokus fase ini
- [ ] [STOP] Trait bound `T: Trait`, `where`, `dyn Trait`, `Box<dyn Trait>`
- [ ] [STOP] `Rc`, `RefCell`, `Arc`, `Mutex`
- [ ] [STOP] `Send`, `Sync`
- [ ] [STOP] `move ||` pada closure untuk thread
- [ ] [SKIP] `Into<String>`, `AsRef<str>` ("terima apa pun yang bisa diubah/dipandang sebagai ini")
- [ ] [SKIP] Deref coercion: kenapa `&String` bisa dipakai sebagai `&str`
- [ ] [NEXT] `Cow<str>` ("pinjam kalau cukup, miliki kalau perlu"; relevan dengan pertanyaan alokasi `String`)
- [ ] [NEXT] `?Sized`, `PhantomData`, `Pin` (jarang, nanti)
- [ ] [NEXT] `unsafe` → pelajari sesuai kebutuhan FFI

## FASE 3: Istilah yang Akan Muncul

- [ ] [SKIP] `#[tokio::main]`, `#[derive(Serialize, Deserialize)]`, `#[serde(rename = "...")]`
- [ ] [STOP] `async fn`, `.await`, `Future` ("nilai yang belum selesai, malas")
- [ ] [STOP] Kenapa `tokio::spawn` butuh `Send + 'static`
- [ ] [SKIP] `tokio::select!`, `tokio::join!`
- [ ] [SKIP] `Path` vs `PathBuf`: sama seperti `&str` vs `String`
- [ ] [NEXT] `Pin<Box<dyn Future>>`
- [ ] [NEXT] `AsRawFd`, `OwnedFd` (file descriptor, saat mulai ke event loop)

## FASE 4: Istilah yang Akan Muncul *(perkiraan)*

- [ ] [SKIP] Macro `delegate_*!` pada smithay-client-toolkit ("sambungkan event ke state saya")
- [ ] [STOP] Trait `Dispatch` / handler: cara menerima event Wayland
- [ ] [SKIP] `QueueHandle`, `EventQueue`, `GlobalList` (objek pembawa; pakai sesuai contoh)
- [ ] [SKIP] Kode hasil `wayland-scanner` (digenerate otomatis, tidak perlu dibaca)
- [ ] [NEXT] Raw window handle / `unsafe` untuk rendering

## FASE 5: Istilah yang Akan Muncul *(perkiraan)*

- [ ] [SKIP] GTK: `glib::clone!`, pola `Rc<RefCell<...>>` di callback
- [ ] [SKIP] relm4: `#[relm4::component]`, `view!`, `ComponentSender`
- [ ] [STOP] Pola Elm di iced: `Message` enum + `update` + `view`
- [ ] [NEXT] Subclassing objek GTK (`ObjectSubclass`) → hanya kalau butuh widget kustom

---

## Log Istilah Parkir

Setiap ketemu istilah asing yang tidak langsung penting, catat di sini.

| Istilah | Ketemu di mana | Kategori | Dipelajari di | Sudah? |
|---|---|:---:|---|:---:|
| `'a` | contoh struct | [NEXT] | Fase 2 | [ ] |
| | | | | [ ] |
| | | | | [ ] |

## Saat Buntu

- [ ] Tulis versi paling sederhana dulu (`clone` dan alokasi bebas)
- [ ] Jalankan `rustc --explain EXXXX` untuk kode error yang muncul
- [ ] Tulis ulang masalahnya dengan kata sendiri (sering langsung ketemu jawabannya)
- [ ] Kalau sudah 20 menit tanpa kemajuan, tanyakan dengan menyertakan **kode + pesan error utuh**
