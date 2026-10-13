# Deep Dive: `String` vs `&str` di Rust

Dokumentasi ini membedah representasi memori, struktur data, dan aturan kepemilikan (ownership) dari tipe data teks di Rust, dengan pemetaan langsung ke konsep C.

## 1. Konsep Fundamental

Di Rust, teks tidak direpresentasikan oleh satu tipe data tunggal. Ada dua entitas yang berbeda secara struktural:

1. **`String`**: Tipe data yang **memiliki** (owns) datanya. Data disimpan di heap, dapat diubah (mutable), dan ukurannya dapat bertambah (growable).
2. **`&str`** (String Slice): Tipe data yang **meminjam** (borrows) datanya. Ini adalah *view* (jendela pandang) ke urutan byte UTF-8 yang sudah ada di suatu tempat (heap, stack, atau read-only memory).

---

## 2. `String`: Owned, Heap-Allocated, Growable

`String` secara internal adalah wrapper dari `Vec<u8>`. Ia menjamin bahwa byte-byte di dalamnya selalu membentuk encoding UTF-8 yang valid.

### Struktur Memori (Stack)

Pada arsitektur 64-bit, variabel `String` menempati **24 byte** di stack, yang terdiri dari 3 *word* (field):

1. `ptr`: Pointer ke data aktual di heap.
2. `len`: Jumlah byte yang saat ini berisi data valid.
3. `cap`: Total kapasitas byte yang telah dialokasikan di heap.

### Padanan 1:1 di C

```c
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// Ekuivalen struktur String di stack
typedef struct {
    uint8_t* ptr;   // Pointer ke heap
    size_t len;     // Panjang data saat ini
    size_t cap;     // Kapasitas alokasi heap
} RustString;

// Ekuivalen String::from("halo")
RustString create_string(const char* literal) {
    size_t literal_len = strlen(literal);
    RustString s;
    s.cap = literal_len;
    s.len = literal_len;
    s.ptr = (uint8_t*)malloc(s.cap); // Alokasi heap
    memcpy(s.ptr, literal, s.len);   // Salin data dari .rodata ke heap
    return s;
}

// Ekuivalen Trait Drop (dipanggil otomatis saat scope berakhir)
void drop_string(RustString* s) {
    free(s->ptr); // Bebaskan memori heap
    s->ptr = NULL;
    s->len = 0;
    s->cap = 0;
}
```

### Fakta Teknis `String`

- **Tidak ada Null Terminator:** Rust tidak menyimpan `\0` di akhir `String`. Panjang string diketahui secara pasti dari field `len`.
- **Realokasi (Growable):** Jika `len` akan melebihi `cap` saat data ditambahkan (misal via `push_str`), Rust akan melakukan realokasi (setara `realloc`, atau `malloc` + `memcpy` + `free`), menyalin data ke blok memori baru yang lebih besar, dan mengupdate `ptr` dan `cap`.
- **Ownership:** Saat variabel `String` keluar dari scope, Rust secara otomatis memanggil `dealloc` (ekuivalen `free`) pada `ptr`. Tidak ada memory leak.

---

## 3. `&str`: Borrowed View, String Slice

`&str` adalah *fat pointer* (pointer gemuk). Ia tidak memiliki data, ia hanya menunjuk ke data yang sudah ada.

### Struktur Memori (Stack)

Pada arsitektur 64-bit, variabel `&str` menempati **16 byte** di stack, terdiri dari 2 *word*:

1. `ptr`: Pointer ke byte pertama dari data.
2. `len`: Panjang byte tersebut.

### Padanan 1:1 di C

*(Catatan: C standar tidak memiliki slice bawaan; ini adalah representasi strukturalnya)*

```c
typedef struct {
    const uint8_t* ptr; // Pointer ke data (bisa di heap, stack, atau .rodata)
    size_t len;         // Panjang data
} RustStrSlice;

// Ekuivalen membuat &str dari String
RustStrSlice borrow_string(const RustString* s) {
    RustStrSlice slice;
    slice.ptr = s->ptr;
    slice.len = s->len;
    return slice; // Tidak ada malloc, tidak ada copy data
}
```

### Fakta Teknis `&str`

- **Zero-Cost Slicing:** Memotong `String` menjadi `&str` (misal `&s[0..5]`) tidak menyalin data ke heap. Ia hanya membuat struct 16 byte di stack yang `ptr`-nya menunjuk ke alamat yang sama dengan `String`, dan `len`-nya disesuaikan.
- **Lokasi Data Fleksibel:** Data yang ditunjuk `&str` bisa berada di:
  - **Heap** (jika di-slice dari `String`).
  - **Read-Only Data (`.rodata`)** (jika dibuat dari string literal seperti `"teks biasa"`).
- **Immutable:** `&str` tidak mengizinkan modifikasi panjang atau isi data yang dirujuk.

---

## 4. Realita UTF-8 dan Aturan Indexing

Ini adalah perbedaan paling krusial dengan C. `String` dan `&str` **bukanlah array of char**. Mereka adalah **array of byte (`u8`)** yang di-encode dalam UTF-8.

### Mengapa tidak bisa di-index langsung? (`s[0]`)

Dalam UTF-8, satu karakter (Unicode Scalar Value) ukurannya bervariasi:

- ASCII (`a`, `1`, `!`) = **1 byte**
- Latin extended (`é`, `ñ`) = **2 byte**
- CJK / Kanji (`中`) = **3 byte**
- Emoji (`🦀`, `😀`) = **4 byte**

Karena ukuran per karakter tidak tetap (variable-width), **mustahil** menghitung offset memori karakter ke-N dalam waktu O(1). Oleh karena itu, Rust **melarang** akses indeks langsung (`s[0]`) pada string.

### Aturan Slicing (`&s[a..b]`)

Anda bisa memotong string menggunakan rentang **byte** (`a..b`), tetapi Rust melakukan **validasi ketat di runtime**:

1. **Bounds Check:** `b` tidak boleh melebihi `len`.
2. **Char Boundary Check:** Indeks `a` dan `b` **harus** jatuh pada batas awal karakter UTF-8 yang valid.

```rust
let s = String::from("a🦀b");
// Byte layout: [0x61] [0xF0, 0x9F, 0xA6, 0x80] [0x62]
// Indeks byte:    0      1    2    3    4         5

let valid = &s[0..1]; // OK: mengambil byte 0 ('a')
let crash = &s[0..2]; // PANIC! Indeks 2 berada di tengah-tengah emoji 🦀.
```

Jika validasi gagal, program akan **panic** (berhenti secara terkontrol dan aman). Ini mencegah terciptanya `&str` yang berisi byte UTF-8 rusak (invalid UTF-8).

---

## 5. Konversi dan Biaya Alokasi

### `&str` ke `String` (Memerlukan Alokasi)

Mengubah slice yang "meminjam" menjadi `String` yang "memiliki" memerlukan alokasi heap baru dan penyalinan data.

```rust
let literal: &str = "halo";              // Data di .rodata (read-only)
let owned: String = literal.to_string(); // malloc + memcpy ke heap
// Alternatif: String::from(literal)
```

### `String` ke `&str` (Zero-Cost)

Mengambil referensi dari `String` untuk mendapatkan `&str` tidak memerlukan alokasi.

```rust
let owned: String = String::from("halo");
let borrowed: &str = &owned; // Hanya membuat fat pointer 16 byte di stack

// Deref coercion: &String otomatis dikonversi menjadi &str saat dibutuhkan
fn cetak(s: &str) {
    println!("{}", s);
}
cetak(&owned); // Valid tanpa konversi eksplisit
```

---

## 6. Ringkasan Perbedaan

| Fitur | `String` | `&str` |
|---|---|---|
| **Kepemilikan** | Memiliki data (Owned) | Meminjam data (Borrowed) |
| **Lokasi Data** | Selalu di Heap | Heap, Stack, atau `.rodata` |
| **Ukuran di Stack** | 24 byte (ptr, len, cap) | 16 byte (ptr, len) |
| **Mutabilitas** | Growable (bisa `push_str`) | Fixed-size view (tidak bisa grow) |
| **Null Terminator** | Tidak ada | Tidak ada |
| **Alokasi Memori** | `malloc` / realokasi | Tidak ada (zero-cost) |
| **Pembersihan** | Otomatis `free` (trait `Drop`) saat scope habis | Tidak melakukan `free` |

---

## Referensi

- The Rust Team. (n.d.). *std::string::String*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/string/struct.String.html
- The Rust Team. (n.d.). *std::primitive::str*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/primitive.str.html
- Klabnik, S., & Nichols, C. (2023). *The Rust Programming Language* (2nd ed.). No Starch Press. https://doc.rust-lang.org/book/
