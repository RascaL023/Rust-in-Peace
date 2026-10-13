# Data Type

Tipe data di Rust pada umumnya mirip dengan bahasa pemrograman lain seperti C, Java, dan Go. Perbedaannya terletak pada sintaks dan beberapa aturan yang diterapkan oleh Rust.

Dokumentasi Rust membagi tipe data menjadi dua kategori utama:

- **Scalar:** merepresentasikan satu nilai.
- **Compound:** mengelompokkan beberapa nilai menjadi satu tipe.

## Scalar

Scalar merepresentasikan tipe data yang memiliki satu nilai. Rust memiliki empat tipe data scalar utama:

1. Integer
2. Floating-point
3. Boolean
4. Character

### Integer

Integer adalah tipe data untuk merepresentasikan bilangan bulat, seperti `-10`, `0`, dan `100`.

Rust menyediakan beberapa tipe integer berdasarkan ukuran bit dan apakah tipe tersebut mendukung bilangan negatif atau tidak.

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Architecture-dependent | `isize` | `usize` |

**Perbedaan Signed dan Unsigned:**

- **Signed (`i`)**: dapat menyimpan bilangan negatif, nol, dan positif.
- **Unsigned (`u`)**: hanya dapat menyimpan nol dan bilangan positif.

Contohnya, `i8` memiliki rentang nilai dari `-128` sampai `127`, sedangkan `u8` memiliki rentang nilai dari `0` sampai `255`.

`isize` dan `usize` memiliki ukuran yang bergantung pada arsitektur target program.

### Integer Literals

Rust mendukung beberapa cara untuk menuliskan literal integer.

| Jenis literal | Contoh |
|---|---|
| Decimal | `98_222` |
| Hexadecimal | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8`) | `b'A'` |

Underscore (`_`) dapat digunakan untuk memisahkan angka agar lebih mudah dibaca. Contohnya, `98_222` memiliki nilai yang sama dengan `98222`.

Literal hexadecimal menggunakan awalan `0x`, octal menggunakan `0o`, dan binary menggunakan `0b`.

Literal byte seperti `b'A'` merepresentasikan nilai byte untuk karakter ASCII tersebut, yaitu `65`, dengan tipe `u8`.

### Integer Overflow

**Integer overflow** terjadi ketika hasil operasi integer melewati rentang nilai yang dapat ditampung oleh tipe datanya.

Sebagai contoh, `u8` hanya dapat menyimpan nilai dari `0` sampai `255`. Jika hasil operasi menghasilkan nilai `256`, nilai tersebut tidak dapat direpresentasikan sebagai `u8` secara langsung.

Perilaku integer overflow di Rust bergantung pada mode kompilasi dan operasi yang digunakan.

#### Debug Mode

Dalam mode debug, Rust secara default memeriksa integer overflow pada operasi aritmetika integer biasa. Jika overflow terjadi, program akan mengalami **panic** saat runtime.

*Panic* adalah kondisi ketika program mengalami kegagalan yang tidak dapat ditangani melalui alur normalnya dan mulai menghentikan eksekusi, kecuali jika panic tersebut ditangani dengan mekanisme yang sesuai.

Contoh:

```rust
fn main() {
    let number: u8 = 255;
    let result = number + 1;

    println!("{result}");
}
```

Dalam mode debug, operasi `255 + 1` akan menyebabkan panic karena hasilnya melewati batas maksimum `u8`.

#### Release Mode

Dalam mode release, yang biasanya dibuat menggunakan perintah `cargo run --release` atau `cargo build --release`, pemeriksaan overflow yang menyebabkan panic pada operasi integer biasa umumnya tidak aktif secara default.

Untuk operasi tersebut, overflow akan menggunakan perilaku **wrapping**: nilai kembali ke awal rentang tipe integer.

Contoh perilaku wrapping untuk `u8`:

| Hasil matematis | Hasil wrapping |
|---:|---:|
| `255` | `255` |
| `256` | `0` |
| `257` | `1` |
| `258` | `2` |

Dengan demikian, nilai `256` akan menjadi `0`, nilai `257` menjadi `1`, dan seterusnya.

Perilaku ini dapat menghasilkan nilai yang tidak sesuai dengan maksud program. Karena itu, jangan mengandalkan overflow untuk mendapatkan hasil tertentu secara tidak sengaja.

#### Menangani Integer Overflow Secara Eksplisit

Rust menyediakan beberapa kelompok metode untuk menentukan perilaku ketika overflow terjadi.

| Metode | Perilaku |
|---|---|
| `wrapping_*` | Menggunakan perilaku wrapping, baik dalam mode debug maupun release. |
| `checked_*` | Mengembalikan `None` jika overflow terjadi, dan hasilnya jika tidak terjadi overflow. |
| `overflowing_*` | Mengembalikan hasil operasi dan sebuah nilai `bool` yang menunjukkan apakah overflow terjadi. |
| `saturating_*` | Membatasi hasil pada nilai minimum atau maksimum tipe data. |

Tanda `*` menunjukkan bahwa terdapat beberapa metode dalam kelompok tersebut. Contohnya, `wrapping_add` digunakan untuk penjumlahan dengan perilaku wrapping.

### Type Annotation

**Type annotation** adalah cara untuk menyatakan tipe data secara eksplisit ketika membuat sebuah binding.

Contoh:

```rust
let age: i32 = 20;
```

Penjelasan:

- `let` digunakan untuk membuat binding.
- `age` adalah nama binding.
- `: i32` merupakan type annotation yang menyatakan tipe data `age`.
- `20` adalah nilai yang diberikan.

Dengan type annotation, kita memberi tahu compiler tipe data yang kita inginkan.

Contoh lainnya:

```rust
let small_number: u8 = 200;
```

Pada contoh tersebut, `small_number` memiliki tipe `u8`, yang dapat menyimpan bilangan dari `0` sampai `255`.

Jika nilai yang diberikan tidak sesuai dengan tipe yang ditentukan, compiler dapat menolak program tersebut.

### Type Inference

**Type inference** adalah kemampuan compiler Rust untuk menentukan tipe data berdasarkan informasi dan konteks yang tersedia, sehingga kita tidak selalu perlu menuliskan tipe secara eksplisit.

Contoh:

```rust
let age = 20;
```

Kita tidak menuliskan tipe `age`, tetapi compiler tetap menentukan tipe datanya.

Untuk literal integer tanpa informasi tipe lain, Rust menggunakan `i32` sebagai tipe default.

Contoh lainnya:

```rust
let temperature = 36.5;
```

Literal angka pecahan seperti `36.5` secara default bertipe `f64`.

Perhatikan perbedaan kedua contoh berikut:

```rust
let age: i32 = 20; // Type annotation
let age = 20;      // Type inference
```

Pada contoh pertama, tipe `i32` ditulis secara eksplisit. Pada contoh kedua, compiler menyimpulkan tipe tersebut dari informasi yang tersedia.

#### Type Inference Berdasarkan Konteks

Compiler juga dapat menentukan tipe berdasarkan tipe data yang sudah diketahui.

```rust
let a: u8 = 10;
let b: u8 = 20;

let result = a + b;
```

Karena `a` dan `b` bertipe `u8`, hasil penjumlahannya juga bertipe `u8`.

Namun, Rust tidak otomatis mengubah satu tipe integer menjadi tipe integer lainnya hanya karena keduanya digunakan dalam operasi yang sama.

```rust
let a: u8 = 10;
let b: i32 = 20;

let result = a + b; // Error: tipe tidak cocok
```

Kode tersebut menghasilkan error karena `a` bertipe `u8`, sedangkan `b` bertipe `i32`.

Rust menerapkan pemeriksaan tipe yang ketat. Jika dua nilai memiliki tipe berbeda, kita perlu melakukan konversi tipe secara eksplisit sebelum mengoperasikannya.

#### Ketika Type Inference Tidak Cukup

Ada situasi ketika compiler tidak memiliki cukup informasi untuk menentukan tipe data yang diinginkan.

Contohnya:

```rust
let number = "42".parse();
```

Compiler tidak mengetahui tipe integer apa yang ingin dihasilkan dari teks `"42"`.

Kita dapat memberikan type annotation untuk menjelaskan tipe yang diinginkan:

```rust
let number: u64 = "42".parse().unwrap();
```

Pada contoh tersebut, `: u64` memberikan informasi tipe yang dibutuhkan compiler.

`parse()` digunakan untuk mencoba mengubah teks menjadi tipe yang sesuai, sedangkan `unwrap()` mengambil hasilnya atau menyebabkan panic jika operasi tersebut gagal. Penanganan error akan dibahas lebih lanjut pada materi tersendiri.

### Ringkasan

| Konsep | Penjelasan |
|---|---|
| Integer | Tipe data untuk bilangan bulat. |
| Signed | Mendukung bilangan negatif dan positif. |
| Unsigned | Tidak mendukung bilangan negatif. |
| Integer overflow | Hasil operasi melewati rentang tipe integer. |
| Panic | Kegagalan yang menyebabkan program menghentikan eksekusi normalnya. |
| Type annotation | Tipe data ditulis secara eksplisit, seperti `: u8`. |
| Type inference | Compiler menyimpulkan tipe berdasarkan informasi yang tersedia. |
| Default integer type | `i32`, jika tipe integer tidak ditentukan oleh konteks lain. |
| Type mismatch | Ketidakcocokan tipe data dalam suatu operasi. |

# Deep Dive: Tuple, Array, dan Slice di Rust

Dokumentasi representasi tipe data komposit di Rust: struktur memori, aturan akses, dan padanan 1:1 ke konsep C. Bagian dari Fase 1 (Inti Bahasa & Ownership).

## 1. Tuple: Heterogen, Fixed-Size

Tuple adalah kumpulan nilai dengan **tipe data yang boleh berbeda** dan **ukuran tetap** yang diketahui saat compile time.

### Aturan Akses
- Tuple **bukan** array: tidak bisa diakses dengan bracket index (`t[0]`) maupun indeks variabel (`t[i]`).
- Akses menggunakan dot notation dengan angka literal: `t.0`, `t.1`, `t.2`.
- Alasan: setiap posisi boleh bertipe berbeda, sehingga compiler harus mengetahui tipe hasil akses secara pasti saat compile time. Angka setelah dot adalah **nama field**, bukan indeks memori.
- Tuple juga bisa dibongkar (destructuring): `let (a, b, c) = t;`

### Struktur Memori
- Disimpan contiguous di stack (layaknya struct); ukuran = jumlah ukuran field + padding alignment.
- Tidak ada metadata runtime (len/cap); panjang dan tipe merupakan bagian dari signature tipe.

### Catatan Ownership
- Mengambil nilai keluar dari tuple (misal `let s = t.0;`) akan **memindahkan (move)** kepemilikan elemen tersebut jika tipenya non-`Copy` (seperti `String`). Tuple mengalami *partial move*.

### Padanan 1:1 di C
```c
// Rust: let t = (10i32, 3.14f64, true);
struct Tuple_i32_f64_bool {
    int32_t _0;
    double  _1;
    bool    _2;
};
struct Tuple_i32_f64_bool t = {10, 3.14, true};
int32_t x = t._0;   // ekuivalen t.0 di Rust
```

---

## 2. Array: Homogen, Fixed-Size, Contiguous

Array adalah kumpulan nilai dengan **tipe data sama** `T` dan **panjang tetap** `N`, ditulis sebagai `[T; N]`.

### Fakta Teknis
- `N` harus diketahui saat compile time dan menjadi bagian dari tipe: `[i32; 3]` dan `[i32; 4]` adalah dua tipe yang berbeda.
- Data disimpan contiguous; secara default dialokasikan di stack (kecuali dibungkus `Box`/`Vec` atau dideklarasikan `static`).
- Tidak ada metadata len/cap runtime; panjang diketahui dari tipe.
- Akses indeks `arr[i]` diizinkan (termasuk indeks variabel) karena semua elemen bertipe sama. Rust menyisipkan **bounds check** di runtime; indeks di luar rentang menyebabkan panic.
- Inisialisasi nilai seragam: `let a = [0; 5];` (lima elemen bernilai 0).

### Padanan 1:1 di C
```c
// Rust: let arr: [i32; 3] = [10, 20, 30];
int32_t arr[3] = {10, 20, 30};
```

---

## 3. Slice (`&[T]`): Borrowed View ke Sequences Contiguous

Slice adalah *view* (jendela pandang) ke sebagian atau seluruh elemen dari koleksi yang **homogen dan contiguous**: array, `Vec`, atau `String`. (`&str` adalah slice khusus: `&[u8]` dengan jaminan validitas UTF-8.)

### Aturan Penting
- Slice **tidak bisa** dibuat dari tuple, karena tuple heterogen sehingga tipe hasil potongan akan ambigu.
- Slice **selalu berupa borrow** (referensi); slice tidak pernah memiliki (own) datanya.
- Struktur: *fat pointer* berukuran 16 byte (arsitektur 64-bit): `ptr` (alamat elemen pertama) + `len` (jumlah elemen).
- Pembuatan slice bersifat zero-cost: tidak ada penyalinan data dan tidak ada alokasi.
- Slicing `&arr[a..b]` melakukan bounds check di runtime.
- Dua varian:
  - `&[T]`: immutable borrow, read-only.
  - `&mut [T]`: mutable borrow, dapat memodifikasi elemen aslinya.

### Padanan 1:1 di C
```c
typedef struct {
    const int32_t* ptr;
    size_t len;
} Slice_i32;          // ekuivalen &[i32]

typedef struct {
    int32_t* ptr;
    size_t len;
} SliceMut_i32;       // ekuivalen &mut [i32]

int32_t arr[4] = {10, 20, 30, 40};
Slice_i32 s = { &arr[1], 2 };   // ekuivalen &arr[1..3] di Rust
```

### Contoh Rust
```rust
let mut arr = [10, 20, 30, 40];

// Slice immutable: read-only
let read_only: &[i32] = &arr[1..3];
println!("{:?}", read_only); // [20, 30]

// Slice mutable: boleh memodifikasi data asli,
// tetapi hanya saat tidak ada borrow lain yang aktif
let writable: &mut [i32] = &mut arr[0..2];
writable[0] = 99; // arr[0] kini bernilai 99
```

---

## 4. Borrow: `&T` vs `&mut T`

Borrow adalah referensi ke data tanpa mengambil alih kepemilikan (ownership).

| | Immutable Borrow | Mutable Borrow |
|---|---|---|
| Sintaks | `&T` / `&[T]` | `&mut T` / `&mut [T]` |
| Hak Akses | Read-only | Read-write |
| Padanan C | `const T*` | `T*` |
| Aturan | Boleh banyak aktif bersamaan | Hanya SATU aktif; tidak boleh ada borrow lain selama ia aktif |

Aturan inti (fondasi Borrow Checker):
**Banyak pembaca ATAU satu penulis. Tidak pernah keduanya pada saat yang sama.**

Alasan desain: mencegah *data race* dan *dangling reference* (misalnya slice yang menunjuk ke memori yang sudah direalokasi atau dibebaskan).

---

## 5. Tabel Ringkasan

| Fitur | Tuple `(A, B, C)` | Array `[T; N]` | Slice `&[T]` |
|---|---|---|---|
| Tipe elemen | Boleh berbeda (heterogen) | Harus sama (homogen) | Harus sama (homogen) |
| Ukuran | Tetap, bagian dari tipe | Tetap, bagian dari tipe | Dinamis (len diketahui runtime) |
| Akses | `t.0` (literal saja) | `arr[i]` (variabel boleh, bounds check) | `s[i]` (bounds check) |
| Kepemilikan | Owns data | Owns data | Borrow (tidak pernah own) |
| Lokasi default | Stack | Stack (contiguous) | Menunjuk ke array/Vec/String |
| Ukuran di stack | Jumlah field + padding | `sizeof(T) * N` | 16 byte (ptr + len) |
| Padanan C | `struct` | `T arr[N]` | `struct { T* ptr; size_t len; }` |

---

## Referensi

- Klabnik, S., & Nichols, C. (2023). *The Rust Programming Language* (2nd ed.). No Starch Press. https://doc.rust-lang.org/book/
- The Rust Team. (n.d.). *Primitive Type tuple*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/primitive.tuple.html
- The Rust Team. (n.d.). *Primitive Type array*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/primitive.array.html
- The Rust Team. (n.d.). *Primitive Type slice*. Rust Standard Library Documentation. https://doc.rust-lang.org/std/primitive.slice.html
