# Manipulasi String di Rust

Cakupan: method-method inti `String` dan `str`, perilaku alokasi masing-masing, dan jebakan umum.
Prasyarat pemahaman: perbedaan `String` (owned, heap) vs `&str` (borrow, fat pointer), UTF-8 variable-width, slice & char boundary.

---

## 1. Prinsip Dasar: Dua Set Method

`String` dan `&str` berbagi sebagian besar method karena `String` melakukan *deref coercion* ke `str`.
Aturan praktis:

- **Method yang mengembalikan potongan/hasil pandang** (`trim`, `find`, `get`, slicing) → ada di `str`, zero-alokasi, return `&str` / `Option<usize>` / `bool`.
- **Method yang mengubah isi di tempat** (`push`, `push_str`, `truncate`, `clear`, `retain`) → hanya ada di `String` (butuh `&mut self`, jadi variabel harus `let mut`).
- **Method yang menghasilkan string baru** (`to_uppercase`, `replace`, `repeat`, `format!`) → return `String` baru, selalu alokasi heap.

Mengetahui kategori ini = mengetahui apakah sebuah baris kode melakukan `malloc` tersembunyi atau tidak.

---

## 2. Membangun dan Menyambung String

### 2.1 `push_str` dan `push` (in-place, disarankan)
```rust
let mut s = String::new();
s.push_str("Sari");   // menambah &str (borrow, tidak mengambil kepemilikan)
s.push(' ');          // menambah satu char
s.push_str("Dewi");
// s == "Sari Dewi"
```
- `push_str(&mut self, string: &str)`: menyalin byte dari argumen ke buffer milik `s`. Jika `len` melebihi `cap`, terjadi realokasi (malloc baru + memcpy + free lama).
- Argumen `&str` berarti penyambungan tidak memindahkan kepemilikan argumen; `&str` boleh dipakai lagi setelahnya.

### 2.2 Operator `+` (trait `Add`) — pahami sebelum memakai
```rust
let a = String::from("Sari");
let b = String::from("Dewi");
let c = a + &b;      // VALID: kiri String, kanan &str
// println!("{a}");  // ERROR: a sudah MOVED ke dalam add()
```
- Implementasi resminya: `impl Add<&str> for String` dengan signature `fn add(mut self, other: &str) -> String`.
- Karena `self` (bukan `&self`), operand kiri **dipindahkan (move)**. Operand kanan wajib `&str`.
- Pola `let x = x.add(...)` berulang (shadowing berantai) secara teknis valid tetapi berisik: setiap baris memindahkan String lama ke `add` dan mengikat hasilnya lagi. `push_str` dengan satu `let mut` menyampaikan niat yang sama dengan lebih jujur.

### 2.3 `format!` (alokasi sekali, paling mudah dibaca)
```rust
let name = "Sari Dewi";
let avg = 82;
let summary = format!("Summary: {name} scored {avg} on average");
```
- Selalu mengalokasikan `String` baru sekali untuk hasil akhir.
- Pilihan tepat saat menyusun teks dari banyak potongan berbeda tipe (angka, bool, dll.) karena memakai trait `Display`, bukan konversi manual.

### 2.4 `repeat`
```rust
let garis = "-".repeat(20); // String baru berisi 20 tanda hubung
```

### Perbandingan biaya
| Cara | Alokasi | Operand kiri sesudah operasi |
|---|---|---|
| `s.push_str(x)` | Mungkin realloc bila cap habis | Tetap valid (butuh `mut`) |
| `a + &b` | Memakai ulang buffer `a` | **Moved** (tidak valid) |
| `format!(...)` | Satu alokasi baru | Tidak relevan (argumen dipinjam) |

---

## 3. Memangkas Whitespace: keluarga `trim`

```rust
let raw = String::from("   Sari Dewi   ");
let clean: &str = raw.trim();       // "Sari Dewi"
let kiri: &str = raw.trim_start();  // "Sari Dewi   "
let kanan: &str = raw.trim_end();   // "   Sari Dewi"
```
- Return `&str` yang **meminjam** dari `raw` → zero-alokasi.
- **Jebakan:** `trim` TIDAK mengubah `raw`. `raw` tetap berisi spasi. Jika lupa mengikat hasilnya (`let x = x.trim();`), pemangkasan hilang begitu saja.
- Varian berpola: `trim_start_matches('-')`, `trim_end_matches(|c: char| c.is_numeric())` (closure masuk materi lanjutan).
- Teknik in-place (tanpa String baru) bila benar-benar butuh membuang ekor dari `String` milik sendiri:
  ```rust
  let mut s = String::from("abc\n");
  s.truncate(s.trim_end().len()); // s == "abc"
  ```

---

## 4. Mencari: `find`, `rfind`, `contains`, `starts_with`, `ends_with`

```rust
let name = "Sari Dewi";
let idx: Option<usize> = name.find(' ');   // Some(4) — indeks BYTE pertama
let last: Option<usize> = name.rfind(' '); // Some(4) — indeks BYTE terakhir
let ada: bool = name.contains("Dew");      // true
let awalan: bool = name.starts_with("Sar"); // true
let akhiran: bool = name.ends_with("ewi");  // true
```
- `find`/`rfind` mengembalikan `Option<usize>`: `Some(indeks_byte)` atau `None` bila tidak ditemukan. Menangani `None` wajib (match / if let / unwrap_or) — ini gerbang menuju materi `Option<T>`.
- Indeks yang dikembalikan dijamin berada pada **char boundary**, sehingga aman dipakai langsung untuk slicing.
- Padanan C: `find` ≈ `strchr` (tapi mengembalikan offset, bukan pointer), `rfind` ≈ `strrchr`, `contains` ≈ `strstr(...) != NULL`.
- **Anti magic number:** kasus inisial "S.D" seharusnya:
  ```rust
  let name = "Sari Dewi";
  let awal = &name[0..1];
  let spasi = name.rfind(' ').unwrap_or(0);      // posisi spasi terakhir
  let akhir = &name[spasi + 1..spasi + 2];       // huruf pertama kata terakhir
  let initials = format!("{awal}.{akhir}");
  ```
  Angka `5` hardcode adalah bug menunggu terjadi begitu nama berubah menjadi "Budi Hartono".

---

## 5. Slicing dan Akses Aman

```rust
let s = String::from("Halo Dunia");
let a: &str = &s[0..4];          // "Halo" — panic runtime jika batas bukan char boundary
let b: Option<&str> = s.get(0..4); // Some("Halo") — None, bukan panic, jika invalid
let c: bool = s.is_char_boundary(4); // true
```
- `&s[a..b]`: validasi bounds + char boundary di runtime; gagal → panic.
- `s.get(a..b)`: kontrak yang sama tetapi kegagalan dikembalikan sebagai `None` (mirip `.get()` pada slice/array).
- `len()` menghitung **byte**, bukan karakter: `"🦀".len() == 4`, `"🦀".chars().count() == 1` (iterator: materi lanjutan).
- Padanan C: `len()` ≈ `strlen` tetapi O(1) karena panjang tersimpan di metadata (`len`), bukan hasil scan mencari `\0`.

---

## 6. Transformasi yang Menghasilkan String Baru

```rust
let s = "sari dewi";
let upper: String = s.to_uppercase();          // "SARI DEWI" (alokasi baru)
let lower: String = s.to_lowercase();
let ganti: String = s.replace("sari", "SARI"); // "SARI dewi" (alokasi baru)
let milik: String = s.to_string();             // &str -> String (alokasi baru)
let angka: Result<i32, _> = "82".parse();      // &str -> tipe lain (Result: materi lanjutan)
```
- Semua method di atas **tidak menyentuh** string sumber; mereka membaca byte sumber dan menulis ke buffer heap baru.
- `replace(from, to)` menerima pola `char` atau `&str`; ada `replacen` untuk membatasi jumlah penggantian.
- `parse()` adalah kebalikan `format!`/`Display`: string → tipe primitif, gagal → `Err` (bukan panic).

---

## 7. Mutasi In-Place (khusus `String`, butuh `let mut`)

| Method | Efek |
|---|---|
| `push(ch)` / `push_str(s)` | Menambah di akhir |
| `pop()` | Hapus char terakhir, return `Option<char>` |
| `remove(idx)` | Hapus char pada indeks byte, return `char`; panic jika bukan char boundary |
| `truncate(n)` | Potong jadi `n` byte pertama; panic jika bukan char boundary |
| `clear()` | Kosongkan (`len = 0`, `cap` tetap) |
| `retain(f)` | Simpan hanya char yang memenuhi predikat |
| `insert(idx, ch)` / `insert_str(idx, s)` | Sisipkan di tengah (menggeser byte) |
| `make_ascii_uppercase()` / `make_ascii_lowercase()` | Ubah huruf ASCII di tempat, tanpa alokasi |

Catatan kapasitas: bila ukuran akhir sudah diketahui, `String::with_capacity(n)` menghindari rangkaian realokasi saat `push_str` berulang.

---

## 8. Perbandingan

```rust
let a = String::from("sari");
assert!(a == "sari");                 // String vs &str langsung bisa (impl PartialEq)
assert!(a.eq_ignore_ascii_case("SARI")); // true, abaikan kasus ASCII
```
- `==` membandingkan isi byte per byte dengan panjang eksplisit; padanan C: `strcmp(a, b) == 0`, tetapi tanpa risiko membaca melewati `\0`.

---

## 9. Cheatsheet Cepat

| Method | Receiver | Return | Alokasi? |
|---|---|---|---|
| `push_str` / `push` | `&mut String` | `()` | Mungkin realloc |
| `+` (`Add`) | `String` (moved) | `String` | Pakai buffer kiri |
| `format!` | — | `String` | Ya, sekali |
| `trim*` | `&str`/`String` | `&str` | Tidak |
| `find` / `rfind` | `&str` | `Option<usize>` | Tidak |
| `contains` / `starts_with` / `ends_with` | `&str` | `bool` | Tidak |
| `get(a..b)` | `&str` | `Option<&str>` | Tidak |
| `&s[a..b]` | `&str` | `&str` (panic bila invalid) | Tidak |
| `to_uppercase` / `to_lowercase` | `&str` | `String` | Ya |
| `replace` | `&str` | `String` | Ya |
| `repeat` | `&str` | `String` | Ya |
| `parse::<T>()` | `&str` | `Result<T, E>` | Tergantung T |
| `len` / `is_empty` | `&str` | `usize` / `bool` | Tidak |
| `truncate` / `clear` / `retain` | `&mut String` | `()` | Tidak |

---

## 10. Jebakan yang Sudah Terbukti di Kode Sendiri

1. **Shadowing berantai dengan `.add()`** — valid tetapi menyembunyikan move di tiap baris dan memaksa rebinding. `push_str` + satu `let mut` lebih jujur dan lebih mudah dibaca.
2. **Magic number pada slice** (`&name[5..6]`) — bekerja hanya untuk satu input tertentu. `find`/`rfind` mengembalikan posisi yang benar untuk input apa pun, dan hasilnya dijamin char boundary.
3. **Mengira `trim`/`replace` mengubah sumber** — keduanya murni (pure): sumber tidak berubah; hasil wajib diikat ke variabel.
4. **`len()` dikira jumlah karakter** — ia jumlah byte; untuk teks non-ASCII hasilnya berbeda dari jumlah karakter yang terlihat.
5. **Lupa bahwa `+` memindahkan operand kiri** — memakai `a` setelah `a + &b` adalah E0382.

---

## 11. Latihan Mandiri (uji tanpa melihat cheatsheet)

1. Dari `let input = String::from("  budi santoso  ");`, hasilkan `String` berisi `"B.S"` (inisial kapital titik) tanpa magic number dan tanpa iterator chain.
2. Dari `let csv = "70,85,90";`, ambil angka tengah sebagai `u32` hanya dengan `find`, slicing, dan `parse`.
3. Buat fungsi `fn ulangi_garis(teks: &str, n: usize) -> String` yang mengembalikan `teks` diulang `n` kali dipisah `" | "` menggunakan `push_str` (bukan `format!`).

<details>
<summary>Kunci arah penyelesaian (buka setelah mencoba)</summary>

1. `let t = input.trim();` → `let spasi = t.find(' ').unwrap_or(0);` → slice `[0..1]` dan `[spasi+1..spasi+2]` → `to_uppercase()` per potongan atau `make_ascii_uppercase()` pada String hasil.
2. `let k1 = csv.find(',').unwrap(); let k2 = csv.rfind(',').unwrap(); let tengah: u32 = csv[k1+1..k2].parse().unwrap();`
3. Loop `for i in 0..n { if i > 0 { out.push_str(" | "); } out.push_str(teks); }`
</details>

---

## Referensi

- The Rust Team. (n.d.). *std::string::String*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/string/struct.String.html
- The Rust Team. (n.d.). *std::primitive::str*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/primitive.str.html
- Klabnik, S., & Nichols, C. (2023). *The Rust Programming Language* (2nd ed., Ch. 8.2: Storing UTF-8 encoded text with strings). No Starch Press. https://doc.rust-lang.org/book/ch08-02-strings.html
