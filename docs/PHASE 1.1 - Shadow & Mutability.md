# Mutability

Secara **default**, variable di Rust adalah **Immutable**(nilainya tidak dapat diubah setelah nilainya ditetapkan).
Konsepnya mirip dengan konstanta. Sejak variable tersebut di beri nilai untuk pertama kalinya, maka 
nilainya tidak bisa diubah sembarangan lagi. Contoh:
```rust
fn main() {
    let x = 20;
    x = 10; -> Bakal error di compile time
    println!("Isi x: {x}");
}
```

Lantas bagaimana jika kita ingin variable tersebut bisa di ubah lagi?
Disini lah kita perlu menambahkan keyword `mut`, alias mutable. Contoh:
```rust
fn main() {
    let mut y = 20;
    println!("Isi awal y: {y}");

    y = 10;
    println!("Isi akhir y: {y}");
}
```

Keyword `mut` ini memberikan tanda bahwa variable ini bersifat **mutable**, alias bisa di ubah lagi
setelah nilai awal di berikan.

## Fun fact
Contoh kode:
```rust
fn main() {
    let x = 20;
    println!("Isi x: {x}");

    let mut y = 20;
    y = 10;
    println!("Isi y: {y}");
}
```
Terlihat normal bukan sebagai pengenalan mutability?
Tapi saat kita check saja:
```bash
Rust/phase-1 on  phase/1 [?]                                                                       via  v1.99.0 
❯ cargo check
    Checking phase-1 v0.1.0 (/home/rascal/Documents/Learn/Rust/phase-1)
warning: value assigned to `y` is never read
 --> src/main.rs:7:17
  |
7 |     let mut y = 20;
  |                 ^^ this value is reassigned later and never used
8 |     y = 10;
  |     ------ `y` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `phase-1` (bin "phase-1") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
```

Kalau kita coba supaya `y` tidak diisi dulu awalnya:
```bash
Rust/phase-1 on  phase/1 [?]                                                                       via  v1.99.0
❯ cargo check
    Checking phase-1 v0.1.0 (/home/rascal/Documents/Learn/Rust/phase-1)
warning: variable does not need to be mutable
 --> src/main.rs:7:9
  |
7 |     let mut y;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `phase-1` (bin "phase-1") generated 1 warning (run `cargo fix --bin "phase-1" -p phase-1` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
```

Ya, compiler Rust sangat ketat, ia sangat ingin kode program itu optimal dan tidak sia sia.
> - Analisis Warning Pertama: **unused_assignments**
> Penyebab: Kamu memberikan nilai 20 kepada y, tetapi langsung menimpanya dengan 10 
> sebelum nilai 20 tersebut sempat digunakan atau "dibaca" (misalnya, ditampilkan dengan println!). 
> Compiler Rust sangat efisien dan menganggap pemberian nilai 20 ini sia-sia.

> - Analisis Warning Kedua: **unused_mut**
> Penyebab: Warning ini muncul jika di suatu percobaan kamu menulis let mut y; 
> dan hanya mengisinya satu kali (misalnya y = 10;). 
> Dalam Rust, jika sebuah variabel hanya diinisialisasi/diisi satu kali, 
> dia tidak perlu ditandai sebagai mut (mutable). 
> Kata kunci mut hanya diperlukan jika kamu benar-benar mengubah nilainya setelah inisialisasi pertama.

Kesimpulanya... Welcome to Rust!

---
# Shadowing

Dari istilahnya, shadow berarti 'menutupi'. Variable lama akan tertutupi sementara oleh variable baru.
'Tertutupi' disini bisa dikatakan tertimpa **sementara**, jadi bukan tertimpa sepenuhnya atau permanen.
Agar lebih jelas, perhatikan kode berikut:
```rust
let x = 5;
let x = x + 1; // Ini shadowing, bukan mutasi
let x = x * 2; 
println!("Nilai x adalah: {x}"); // Output: 12
```

Disini mungkin kita terbiasa berpikir kalau variable dengan nama yang sama tidak bisa dibuat 2 kali.
Di Rust, hal tersebut mungkin terjadi, namun bukan benar benar membuat variable baru.

Contoh yang lebih ekstrem:
```rust
let spaces = "   ";       // `spaces` bertipe `&str` (string slice)
let spaces = spaces.len(); // `spaces` yang baru bertipe `usize` (integer)
```

Variable `spaces` yang awalnya bertipe &str, bisa berganti menjadi integer dalam shadowing.

```text
Bagaimana shadowing dari sudut pandang compiler?
```

Ketika compiler Rust melihat let kedua dengan nama yang sama, inilah yang terjadi secara teknis:
- Binding Baru, Bukan Mutasi (Overwrite)
Saat kamu menggunakan let lagi, Rust tidak mengubah nilai di alamat memori variabel yang lama. 
Sebaliknya, compiler membuat binding (ikatan) nama variabel yang benar-benar baru di symbol table (tabel simbol) compiler. 
Variabel lama masih ada secara konseptual, tetapi namanya "tertutup" (shadowed) oleh variabel baru tersebut. 
Setiap kali kamu memanggil x setelahnya, compiler akan merujuk ke binding yang paling baru dibuat.

- Kebebasan Mengubah Tipe Data
Karena ini dianggap sebagai variabel baru, Rust mengizinkan kamu mengubah tipe datanya. 
Ini adalah keunggulan utama shadowing yang tidak bisa dilakukan oleh variabel mut.

- Manajemen Memori dan Drop
Bagaimana dengan memori variabel lama? Rust tetap menjaga keamanan memori:
 - Jika tipe datanya adalah Copy type (seperti i32, bool), nilainya disalin, dan tidak ada masalah kepemilikan (ownership).
 - Jika tipe datanya adalah Owned type (seperti String), dan kamu memindahkan (move) nilainya ke variabel baru, 
 variabel lama dianggap "sudah dipindahkan" (moved from). Saat scope berakhir, Rust hanya akan menjalankan drop 
 (pembersihan memori) pada variabel yang masih memegang ownership, sehingga tidak ada double free atau kebocoran memori.

## Shadowing vs Mutasi


| Fitur              | let (Shadowing)                                       | let mut (Mutasi) |
|  ------            | -----------------                                     | ------------------ |
| Kata Kunci         | Menggunakan let berulang kali                         | Menggunakan let mut sekali |
| Tipe Data          | Boleh berubah                                         | Tidak boleh berubah |
| Konsep             | Membuat variabel baru yang menyembunyikan yang lama   | Mengubah nilai variabel yang sama di tempat |
| Kasus Penggunaan   | "Transformasi data                                    | "Mengupdate nilai secara inkremental |
|                    | (misal: parsing string jadi angka lalu angka jadi operasi lain" | (misal: counter loop, mengubah state)" |

```text
Apakah variable lama yang telah tertutup tersebut masih bisa diakses?
```

Ya, variable lama masih bisa diakses. Kunci nya ada di `scope`.
- **Jika di scope yang sama**: Variable yang lama tersebut mati, tak bisa diakses. 
Umumnya di pakai untuk transformasi data, seperti mengubah tipe datanya.
- **Jika di scope yang beda**: Variable luar atau lama akan disembunyikan sementara, hingga scope berakhir.

Contohnya:
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Nilai x di dalam scope inner: {x}"); // -> 12
    }
    println!("Nilai x di luar scope: {x}"); // -> 6
}
```

> Pada awalnya, x bernilai 5, kemudian di shadow dengan x + 1, dimana x sekarang menjadi 6.
Kemudian eksekusi masuk ke inner scope, di dalamnya x di shadow menjadi x * 2, yang mana menjadi 12.
Lalu di cetak di inner. Karena nilai saat ini 12, maka output menampilkan 12.
Setelah scope berakhir, nilai x bukan lagi 12. Karena scope sudah berakhir, maka x di inner scope tersebut sudah
dihancurkan, tidak bisa diakses dari luar langsung.
Akibatnya, shadow dari varieble x yand diluar sudah tidak tertutupi oleh x di inner scope.
Sehingga menjadi kan nilai x sudah tidak lagi tertutupi oleh x di dalam inner scope.
