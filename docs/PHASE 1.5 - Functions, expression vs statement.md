# Fungsi, Expression vs Statement, dan Control Flow

## 1. Fungsi (Functions)
Di Rust, fungsi dideklarasikan dengan kata kunci `fn`. Berbeda dengan Go atau C, Rust sangat ketat mengenai tipe data dan eksplisitas dalam pengembalian nilai (*return value*).

### Deklarasi Dasar
```rust
// Parameter harus dideklarasikan dengan tipe datanya (prefix)
// Tipe kembalian dideklarasikan setelah panah `->`
fn nama_fungsi(param1: i32, param2: i32) -> i32 {
    // Body fungsi
}
```

### Aturan Pengembalian Nilai (The Return Rules)
Rust **tidak memiliki *implicit return*** seperti di Go (di mana variabel bernama bisa langsung di-return). Di Rust, ada dua cara mengembalikan nilai:

1. **Eksplisit dengan `return`:** Menghentikan eksekusi fungsi dan mengembalikan nilai.
2. **Implisit dengan *Trailing Expression*:** Nilai dari baris terakhir dalam blok fungsi (tanpa diakhiri titik koma `;`) akan otomatis menjadi nilai kembalian. **(Ini adalah cara idiomatic/terbaik di Rust).**

```rust
// Cara 1: Eksplisit (Kurang idiomatic untuk kasus sederhana)
fn tambah_a(a: i32, b: i32) -> i32 {
    return a + b; 
}

// Cara 2: Trailing Expression (Idiomatic Rust)
fn tambah_b(a: i32, b: i32) -> i32 {
    a + b // Tidak ada titik koma!
}
```

---

## 2. Statement vs Expression (Konsep Paling Krusial)
Ini adalah perbedaan paradigma terbesar antara Rust dan bahasa C-family. Memahami ini akan mencegah 80% error kompilasi awal Anda.

| Konsep | Definisi | Menghasilkan Nilai? | Contoh |
| :--- | :--- | :---: | :--- |
| **Statement** | Instruksi yang *melakukan* sesuatu. | ❌ Tidak | `let x = 5;`, `if`, `for`, `while`, `return` |
| **Expression** | Instruksi yang *dievaluasi* menjadi nilai. | ✅ Ya | `5`, `x + 1`, `fungsi()`, `{ blok_kode }` |

### Peran Titik Koma (`;`)
Titik koma di Rust bukan sekadar penutup baris seperti di C/Java. **Menambahkan `;` di akhir sebuah *expression* akan mengubahnya menjadi *statement*.** Artinya, ia akan membuang nilai yang dihasilkannya dan mengembalikan *Unit Type* `()` (setara dengan `void`).

```rust
fn contoh_expression() -> i32 {
    5 + 5 // Expression. Mengembalikan 10.
}

fn contoh_statement() -> () { // Tipe kembalian () / Unit
    5 + 5; // Ditambah `;`. Berubah jadi statement. Mengembalikan ()
}
```

---

## 3. Control Flow sebagai Expression
Di C, Java, atau Go, `if`, `match`, dan `loop` adalah *statement*. Anda tidak bisa menulis `int x = if (true) 5;`. 
**Di Rust, hampir semua control flow adalah Expression.** Mereka bisa dievaluasi dan menghasilkan nilai.

### A. `if` sebagai Expression
Karena `if` menghasilkan nilai, ia bisa langsung digunakan untuk inisialisasi variabel.
```rust
let kondisi = true;

// Idiomatic: Menggunakan if sebagai expression
let angka = if kondisi { 5 } else { 10 }; 

// ❌ SALAH: Cabang if dan else WAJIB memiliki tipe data yang sama.
// let angka = if kondisi { 5 } else { "sepuluh" }; // Error: mismatched types
```
*Catatan Penting:* Jika Anda tidak menyertakan `else`, dan kondisi `if` bernilai `false`, maka `if` tersebut dievaluasi menjadi `()` (Unit type).

### B. `loop` sebagai Expression
Anda bisa menghentikan `loop` tak terbatas dan langsung mengembalikan nilai menggunakan `break <value>`.

```rust
let mut counter = 0;

// loop dievaluasi dan nilainya disimpan di 'result'
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // break membawa nilai 20
    }
};

assert_eq!(result, 20);
```

---

## 4. 🧠 Mental Model Shift (Untuk Dev C/Go/Java)

1. **Lupakan "Assignment di dalam If":** Di C/Java, `if (x = getValue())` adalah hal biasa. Di Rust, `let` adalah *statement*, jadi `if (let x = getValue())` tidak valid sebagai expression. Anda harus memisahkannya atau menggunakan `match`/`if let`.
2. **Blok Kode adalah Expression:** Di Rust, `{ ... }` adalah sebuah expression. Nilai terakhir di dalam blok tersebut adalah nilai dari blok itu sendiri.
   ```rust
   let y = {
       let x = 3;
       x + 1 // Nilai blok ini adalah 4
   };
   ```
3. **Titik Koma adalah "Pembunuh Nilai":** Jika compiler mengeluh bahwa fungsi Anda mengembalikan `()` padahal seharusnya `i32`, 99% masalahnya adalah Anda tidak sengaja menambahkan `;` di baris terakhir fungsi Anda.

---

## 5. 📝 Self-Reflection Prompts (Uji Pemahaman)
*Gunakan pertanyaan ini untuk menguji pemahaman Anda sebelum lanjut ke Fase 2. Cobalah jawab tanpa melihat dokumentasi!*

1. Mengapa kode ini error, dan bagaimana cara memperbaikinya tanpa mengubah logika `if`?
   ```rust
   fn dapatkan_nilai(kondisi: bool) -> i32 {
       if kondisi {
           42
       }
   }
   ```
2. Apa tipe data dari variabel `x` pada kode di bawah ini? Jelaskan alasan Anda berdasarkan konsep statement/expression.
   ```rust
   let x = {
       let y = 5;
       y + 5;
   };
   ```
3. Bagaimana cara Anda menulis ulang fungsi rekursif Fibonacci menjadi iteratif menggunakan `loop` sebagai expression?

---

## 📚 Referensi Akademik
Untuk pendalaman lebih lanjut dan verifikasi konsep, rujuk dokumentasi resmi berikut:

* Klabnik, S., & Nichols, C. (2023). *The Rust Programming Language* (Chapter 3: Common Programming Concepts). No Starch Press. Diakses dari https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
* Mozilla & Rust Contributors. (2023). *The Rust Reference: Expressions*. Diakses dari https://doc.rust-lang.org/reference/expressions.html
