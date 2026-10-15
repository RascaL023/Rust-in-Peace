# `Option<T>` & `Result<T, E>`

> **Ringkasan satu kalimat:** Rust tidak punya `null` dan tidak punya `exception`;
> keduanya digantikan oleh *enum* yang memaksa penanganan kasus di **compile-time**.

| Konsep | Pertanyaan yang dijawab | Variant |
|---|---|---|
| `Option<T>` | “Nilainya **ada** atau tidak?” | `Some(T)` / `None` |
| `Result<T, E>` | “Operasinya **berhasil** atau gagal, dan kenapa?” | `Ok(T)` / `Err(E)` |

Keduanya ada di **prelude** → tidak perlu `use` apa pun.

---

## 1. `Option<T>`

### 1.1 Definisi
```rust
enum Option<T> {
    Some(T), // ada nilai bertipe T
    None,    // tidak ada nilai
}
```

`None` **bukan** `null`. Ia adalah *variant* yang tetap bertipe `Option<T>`,
sehingga kompilator tahu bahwa nilai itu wajib dicek sebelum dipakai.

### 1.2 Aturan emas
> Kamu **tidak bisa** memakai isi `Option<T>` tanpa membukanya dulu.

```rust
let a: Option<i32> = Some(5);
// let b = a + 1;  // ❌ compile error: tidak bisa menambah Option<i32>
let b = a.unwrap() + 1; // ✅ (tapi lihat §1.5 soal unwrap)
```

> 💡 *Fun fact:* untuk banyak tipe (mis. `&T`, `Box<T>`), `Option<T>` **tidak**
> menambah ukuran memori berkat *niche optimization*. Abstraksi ini gratis.

### 1.3 Tiga cara membuka `Option`

```rust
fn cari(data: &[i32], target: i32) -> Option<usize> {
    for (i, &v) in data.iter().enumerate() {
        if v == target { return Some(i); }
    }
    None
}

let data = vec![10, 20, 30];

// (a) match — paling eksplisit, wajib menangani BOTH branch
match cari(&data, 20) {
    Some(i) => println!("indeks {i}"),
    None    => println!("tidak ketemu"),
}

// (b) if let — ringkas, hanya peduli satu branch
if let Some(i) = cari(&data, 20) {
    println!("indeks {i}");
}

// (c) let else — early-exit, menjaga indentasi tetap dangkal
let Some(i) = cari(&data, 20) else {
    println!("tidak ketemu");
    return;
};
println!("indeks {i}"); // i sudah bertipe usize di sini
```

### 1.4 Kombinator (hindari `match` bertingkat)

| Method | Perilaku |
|---|---|
| `.map(f)` | `Some(x)` → `Some(f(x))`; `None` → `None` |
| `.and_then(f)` | sama, tapi `f` harus mengembalikan `Option` (untuk *chaining*) |
| `.filter(p)` | `Some(x)` yang lolos `p` → tetap `Some`; gagal → `None` |
| `.unwrap_or(d)` | nilai `Some`, atau `d` jika `None` |
| `.unwrap_or_else(\|\| d)` | sama, tapi `d` dihitung **lazily** |
| `.ok_or(e)` | konversi ke `Result`: `Some(x)` → `Ok(x)`, `None` → `Err(e)` |
| `.is_some()` / `.is_none()` | cek tanpa membuka |

```rust
let s: Option<&str> = Some("  42  ");
let n: Option<i32> = s
    .map(|t| t.trim())
    .and_then(|t| t.parse::<i32>().ok()); // Option<i32> = Some(42)
```

### 1.5 `unwrap` / `expect` — jebakan klasik
- `.unwrap()` → **panic** jika `None`. Program mati.
- `.expect("pesan")` → sama, tapi panic membawa pesan yang kamu tulis.

> [!NOTE]
> *Unwrap tetap mengikuti aturan move. Bagaimana dengan borrowing? Pelajari as_ref* 

> 🚩 **Aturan praktis:** `unwrap`/`expect` boleh di *test*, *prototype*, atau saat
> kamu punya **invariant** yang menjamin bukan `None`. Di *library* / *production* → hindari.

---

## 2. `Result<T, E>`

### 2.1 Definisi
```rust
enum Result<T, E> {
    Ok(T),  // sukses, berisi nilai T
    Err(E), // gagal, berisi penjelasan E
}
```
`E` bebas bertipe apa pun: `String`, `std::io::Error`, atau *enum* error bikinanmu sendiri.

### 2.2 Contoh dasar
```rust
fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("pembagi tidak boleh nol"));
    }
    Ok(a / b)
}

match bagi(10, 0) {
    Ok(v)    => println!("hasil {v}"),
    Err(why) => println!("gagal: {why}"),
}
```

### 2.3 Operator `?` — propagasi error
`?` melakukan dua hal sekaligus:
1. Jika `Ok(v)` / `Some(v)` → **ekstrak** `v`, lanjut jalan.
2. Jika `Err(e)` / `None` → **return segera** dari fungsi saat itu juga.

```rust
use std::fs;

// ? hanya sah jika fungsi pengembalian-nya Result atau Option
fn baca_dan_kali2(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let isi   = fs::read_to_string(path)?;   // io::Error  -> otomatis dikonversi
    let num: i32 = isi.trim().parse()?;      // ParseIntError -> otomatis dikonversi
    Ok(num * 2)
}
```

**Syarat pemakaian `?`:**
- Fungsi pemanggil **harus** mengembalikan `Result` atau `Option` (bukan `()`).
- Tipe error harus **cocok** atau **bisa dikonversi** (`From`), itulah kenapa
  `Box<dyn std::error::Error>` sering dipakai sebagai “waduk umum” saat belajar.

`main` juga boleh mengembalikan `Result`:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match baca_dan_kali2("angka.txt") {
        Ok(v)  => println!("hasil {v}"),
        Err(e) => eprintln!("gagal: {e}"),
    }
    Ok(())
}
```

### 2.4 Kombinator

| Method | Perilaku |
|---|---|
| `.map(f)` | transform nilai di `Ok`, `Err` diteruskan |
| `.map_err(f)` | transform error, `Ok` diteruskan |
| `.and_then(f)` | *chaining*; `f` mengembalikan `Result` baru |
| `.unwrap_or(d)` / `.unwrap_or_else(\|\| d)` | fallback saat `Err` |
| `.ok()` | konversi ke `Option`: `Ok(x)` → `Some(x)`, `Err(_)` → `None` |
| `.is_ok()` / `.is_err()` | cek tanpa membuka |

### 2.5 Konversi bolak-balik
```rust
let o: Option<i32> = Some(7);
let r: Result<i32, &str> = o.ok_or("kosong")?; // Option -> Result

let back: Option<i32> = r.ok();                 // Result -> Option (error dibuang)
```
> ⚠️ `.ok()` **membuang informasi error**. Pakai hanya kalau alasan kegagalan memang tidak relevan.

---

## 3. `Option` atau `Result`? — Tabel Keputusan

| Situasi | Pilih | Alasan |
|---|---|---|
| Data memang **boleh tidak ada** (lookup, `HashMap::get`, `slice::first`) | `Option<T>` | Bukan kegagalan, hanya ketiadaan |
| Ada **alasan kegagalan** yang perlu dilaporkan | `Result<T, E>` | Pemanggil butuh tahu *kenapa* |
| Parsing, I/O, jaringan, validasi input | `Result<T, E>` | Selalu berpotensi gagal & error-nya informatif |
| Internal fungsi, kegagalan sudah ditangani | nilai biasa / `Option` | Jangan membocorkan `Result` yang tidak berguna |

> **Cara ingat:** `None` = “kosong, dan itu normal”.
> `Err` = “sesuatu **salah**, dan ini penjelasannya”.

---

## 4. Pesan error kompilator yang paling sering muncul

| Pesan | Arti | Solusi |
|---|---|---|
| `mismatched types: expected Option<i32>, found i32` | Lupa membungkus dengan `Some`/`Ok` | `return Some(x);` |
| `no method named X found for enum Option<i32>` | Memakai `Option` seolah nilainya langsung | Buka dulu: `match` / `if let` / kombinator |
| ``the `?` operator can only be used in a function that returns `Result` or `Option` `` | `?` di fungsi bertipe kembali `()` | Ubah signature fungsi, atau tangani dengan `match` |
| `unused `Result` that must be used` | `Result` diabaikan (atribut `#[must_use]`) | Tangani, atau `.let _ = ...;` bila memang sengaja |

---

## 5. Checklist Anti-Pattern

- [ ] `.unwrap()` / `.expect()` di jalur produksi → ganti `?` atau `match`.
- [ ] Mengembalikan `Result<T, ()>` → informasi error hilang; pakai `String`/error khusus.
- [ ] Memakai `Option` padahal ada alasan kegagalan → seharusnya `Result`.
- [ ] `match` bersarang 3+ level → ganti dengan `.and_then()` / `let else`.
- [ ] Panik karena `None`, lalu “ditambal” dengan `if x.is_none()` sebelum setiap akses → pakai kombinator.

---

## 6. Cheat-Sheet Cepat

```text
Option<T>            Result<T,E>
─────────────────────────────────────
Some(v)   <──────►   Ok(v)
None      <──────►   Err(e)

.map / and_then      .map / and_then
                     .map_err        (khusus Result)
.unwrap_or(d)        .unwrap_or(d)
.ok_or(e) ─────────► .ok() ─────────► (kembali ke Option)

? (di fungsi -> Option<T>)   ? (di fungsi -> Result<T,E>)
```

---

## 7. 🔍 Checkpoint Pemahaman (jawab sendiri sebelum lanjut)

1. Fungsi `ambil_user(id: u64)`: kalau user-nya memang tidak terdaftar,
   itu *ketiadaan* atau *kesalahan*? Implikasinya ke pilihan `Option` vs `Result`?
2. Kenapa `.unwrap()` di library dianggap berbahaya, padahal di `#[test] dianggap wajar?
3. Fungsi `f() -> i32` berisi `g()?` di mana `g() -> Result<i32, io::Error>`.
   Kenapa ini gagal kompilasi, dan dua cara berbeda apa yang bisa memperbaikinya?

---

## 8. Referensi

**APA**
> Klabnik, S., & Nichols, C. (2023). *The Rust programming language* (2nd ed.). No Starch Press. — Bab 6 (*Enums and Pattern Matching*) & Bab 9 (*Error Handling*).

**MLA**
> Klabnik, Steve, and Carol Nichols. *The Rust Programming Language*. 2nd ed., No Starch Press, 2023.

**Dokumentasi API**
> Rust Core Team. (n.d.). *Option in std — Rust*. https://doc.rust-lang.org/std/option/
> Rust Core Team. (n.d.). *Result in std — Rust*. https://doc.rust-lang.org/std/result/
