# 🗺️ Proyek Fase 1: Labirin Terminal

> Game labirin di terminal yang dibangun **bertahap**. Tiap tahap mengubah cara kerja programnya
> (data, struktur, atau penanganan error), jadi kamu berlatih konsep yang sama dari sudut berbeda.
> **Tidak ada petunjuk langkah demi langkah dan tidak ada kunci jawaban.** Spesifikasinya eksplisit, solusinya milikmu.

---

## 📜 Aturan Main untuk Dirimu Sendiri

- Kerjakan **satu tahap sampai semua kriteria selesai dicentang**, baru lanjut.
- Satu proyek Cargo per tahap (`cargo new tahap1`, lalu salin ke `tahap2`, dst.) supaya bisa dibandingkan.
- Boleh membaca *The Book* kapan saja. **Jangan minta AI menulis solusinya.** Minta AI hanya me-review setelah kamu selesai.
- Buntu lebih dari 20 menit? Tanyakan dengan menyertakan **kode + pesan error utuh**.
- `cargo fmt` dan `cargo clippy` harus bersih (tanpa warning) di akhir tiap tahap.
- Teks pesan di contoh boleh kamu ubah. **Perilakunya yang harus sama.**

---

## 🔒 Aturan Kode Global

**✅ Boleh:**
- Semua yang ada di checklist Fase 1 yang sudah dicentang
- `println!`, `print!` (hanya untuk menggambar baris peta)
- `loop`, `while`, `for i in 0..n`, `break`, `continue`
- `io::stdin().read_line(&mut buf)`, `.trim()`
- Method sederhana pada angka dan Option: `.min()`, `.max()`, `.saturating_sub()`, `.is_some()`, `.is_none()`, `.take()`
- Operator `as` untuk konversi antar tipe angka
- Satu file `src/main.rs` saja

**🚫 Dilarang (sampai Tahap 5):**
- `Vec`, `HashMap`, `String` yang disimpan di struct
- Closure (`|x| ...`) dan method iterator (`.iter()`, `.map()`, `.collect()`, `.chars()`, `.split...()`)
- `.clone()`, `.to_string()`, `.to_owned()`, `String::from(...)`, `format!`
  → Setiap kali tergoda memakainya, itu sinyal untuk bertanya: **"siapa pemilik data ini?"**
- `#[derive(...)]`, membuat trait sendiri, `impl Trait`, generics buatan sendiri
- Lifetime `'a`, `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`, `unsafe`
- `const` dan `static` (tulis angka literal; mengulang angka itu wajar, merapikannya urusan Fase 2)
- `mod`, crate eksternal, `std::process::exit` (program berakhir dengan keluar dari loop)
- `unwrap()` dan `expect()`. **Pengecualian:** `expect` hanya pada `read_line` di Tahap 1-4. Mulai Tahap 5 dilarang total.
- `print!` untuk prompt. Gunakan `println!`, karena `print!` butuh flush yang melibatkan trait (materi nanti).

**🧪 Berlaku di semua tahap:**
- Program **tidak boleh panic** pada input apa pun: baris kosong, spasi saja, huruf besar, teks panjang.
- Logika gerak **tidak boleh mengandalkan fakta bahwa tepi peta adalah dinding.** Uji dengan mengubah satu nilai tepi peta menjadi `0`. Hasilnya harus "di luar peta", bukan panic.

---

## 🧭 Peta Tahap

| Tahap | Nama | Fokus utama | Yang berubah |
|:---:|---|---|---|
| 1 | Jalan-jalan | tuple, array 2D, fungsi, `Option` | Game dasar dengan angka mentah |
| 2 | Model yang benar | `enum`, `match` tuntas, `&mut` | Angka & string diganti enum, ada kunci & pintu |
| 3 | Dunia punya state | `struct`, `impl`, `&self` / `&mut self` | Ada HP, jebakan, ramuan, dan hasil gerak berupa enum |
| 4 | Inventori | `Option` di array, `.take()`, ownership | Barang berpindah pemilik: peta → tas → hilang |
| 5 | Input yang waras | `Result`, `?`, enum error | Input diketik 2 langkah, semua kegagalan jadi nilai |
| 6 🔒 | Konversi otomatis | `impl From`, `Display` | Hapus konversi manual dari Tahap 5 |
| 7 🔒 | Ekosistem | `thiserror`, `anyhow` | Ganti dengan crate |

🔒 = kerjakan **hanya setelah** materi trait dasar di checklist selesai.

---

# 🌱 TAHAP 1: Jalan-jalan

**Tujuan belajar:** tuple, array 2D, fungsi, `Option`, `match` pada `&str`, borrowing `&`.

**Hanya ini yang dipakai:** primitif (angka, `bool`), tuple, array, fungsi. **Belum ada `enum` dan `struct`.**

### Data yang disediakan
Peta 8 kolom × 6 baris, diakses dengan `map[baris][kolom]`:

```rust
[
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 2, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
]
```
Kode: `0` lantai, `1` dinding, `2` pintu keluar. Pemain mulai di **baris 1, kolom 1**.

### Aturan permainan
1. Tiap putaran: gambar peta, tampilkan `Langkah: N`, minta satu perintah.
2. Perintah (setelah `trim`): `n` = baris−1, `s` = baris+1, `e` = kolom+1, `w` = kolom−1, `q` = keluar.
3. Gambar: `#` dinding, `.` lantai, `E` pintu keluar, `@` pemain.
4. Gerak ke dinding → pesan `Tabrak dinding!`. Posisi dan langkah tidak berubah.
5. Gerak keluar peta → pesan `Di luar peta.` Posisi dan langkah tidak berubah.
6. Gerak ke lantai → pindah, langkah +1.
7. Gerak ke pintu keluar → pindah, langkah +1, tampilkan pesan menang beserta jumlah langkah, program selesai.
8. Perintah tidak dikenal (termasuk kosong) → `Perintah tidak dikenal.` Langkah tidak berubah.
9. `q` → `Sampai jumpa.` lalu program selesai.

### Aturan kode tahap ini
- Minimal 3 fungsi terpisah: **menggambar peta**, **menghitung posisi tujuan**, **memutuskan hasil gerak**.
- Fungsi penghitung posisi tujuan mengembalikan `Option<(usize, usize)>`. `None` berarti keluar peta.
- Peta tidak boleh disalin saat dikirim ke fungsi. Kirim sebagai referensi.
- Posisi pemain disimpan sebagai tuple `(usize, usize)`.

### Contoh tampilan
`>` menandai ketikan pengguna.
```
########
#@..#..#
###.#.##
#......#
#.####E#
########
Langkah: 0
Perintah (n/s/e/w/q):
> n
Tabrak dinding!
```

### ✅ Kriteria selesai
- [ ] Jalur tercepat ke pintu keluar menghasilkan tepat **8 langkah**
- [ ] `n` di awal menghasilkan `Tabrak dinding!` dan langkah tetap 0
- [ ] Baris kosong, `N` huruf besar, dan `east` tidak membuat program panic
- [ ] Mengubah satu nilai tepi peta jadi `0` dan berjalan keluar menghasilkan `Di luar peta.` (tidak panic)
- [ ] `cargo clippy` bersih

### 🧠 Uji diri (jawab dengan kata sendiri)
- Kenapa fungsi menggambar peta cukup menerima `&`?
- Apa yang terjadi pada `usize` kalau dikurangi dari 0, dan bagaimana kamu mencegahnya?

---

# 🌿 TAHAP 2: Model yang Benar

**Tujuan belajar:** `enum` dengan data, `match` yang menuntut semua kasus, `&mut`.

### Yang berubah dari Tahap 1
- Angka mentah dan string perintah diganti **enum**.
- Ada **kunci** dan **pintu terkunci** (peta baru, jalannya berbeda).
- Status baru: apakah pemain punya kunci.

### Data yang disediakan
Kode baru: `3` kunci, `4` pintu terkunci. Pemain mulai di **baris 1, kolom 1**.

```rust
[
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1, 3, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 4, 2],
    [1, 0, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
]
```
Peta tetap disimpan sebagai `[[u8; 8]; 6]`. Konversi ke enum hanya terjadi **saat dibaca**, lewat satu fungsi.

### Aturan permainan
1. Perintah sama seperti Tahap 1.
2. Gambar tambahan: `K` kunci, `D` pintu.
3. Menginjak kunci → kunci diambil, pesan `Kamu mengambil kunci!`, dan **kunci hilang dari peta** (kotak jadi lantai).
4. Pintu tanpa kunci → `Pintu terkunci.` Tidak pindah, langkah tidak berubah.
5. Pintu dengan kunci → `Pintu terbuka.` Pemain pindah ke kotak pintu, langkah +1. Kunci tidak habis.
6. Baris status: `Langkah: N | Kunci: ya/tidak`.
7. Aturan dinding, luar peta, pintu keluar, dan `q` sama seperti Tahap 1.

### Aturan kode tahap ini
- Enum wajib: `Tile` (Floor, Wall, Exit, Key, Door), `Direction` (North, South, East, West), dan `Command` (`Go(Direction)`, `Quit`).
- `fn tile_from_code(u8) -> Tile` adalah satu-satunya tempat angka diterjemahkan.
- `fn parse_command(&str) -> Option<Command>`: `None` untuk perintah tidak dikenal.
- Posisi tujuan tetap `Option<(usize, usize)>`.
- `match` atas `Tile`, `Direction`, dan `Command` **dilarang memakai `_ =>`.** Tulis semua varian. Pengecualian: `match` atas `&str` di `parse_command`.
- Status "punya kunci" disimpan sebagai `bool` dan diubah lewat `&mut`, bukan variabel global.
- Peta diubah (kunci dihapus) lewat `&mut` ke peta.

### Contoh tampilan
```
########
#@..#K.#
###.#.##
#.....DE
#.######
########
Langkah: 0 | Kunci: tidak
```

### ✅ Kriteria selesai
- [ ] Jalur tercepat menghasilkan tepat **12 langkah**
- [ ] Menginjak pintu tanpa kunci → `Pintu terkunci.` dan langkah tidak bertambah
- [ ] Setelah kunci diambil, kembali ke kotak kunci tidak mengambilnya lagi (kotaknya sudah lantai)
- [ ] Menambah satu varian dummy ke `Tile` membuat compiler error di semua `match`. Kamu paham kenapa itu bagus. Lalu hapus lagi.
- [ ] `cargo clippy` bersih

### 🧠 Uji diri
- Apa keuntungan `parse_command` mengembalikan `Option<Command>` dibanding mengembalikan `Command` dengan varian `Unknown`?
- Kenapa fungsi yang mengambil kunci butuh `&mut`, sedangkan yang menggambar cukup `&`?

---

# 🌳 TAHAP 3: Dunia Punya State

**Tujuan belajar:** `struct`, `impl`, `&self` vs `&mut self`, enum berisi data, hasil sebagai enum.

### Yang berubah dari Tahap 2
- Variabel yang berserakan dikumpulkan ke **struct**. Logika pindah ke **method**.
- Ada **HP**, **jebakan**, dan **ramuan**.
- Hasil tiap gerakan dikembalikan sebagai **enum** yang diputuskan oleh `main`.

### Data yang disediakan
Kode baru: `5` jebakan (damage 2), `6` ramuan (pulih 3). Pemain mulai di **baris 1, kolom 1**.

```rust
[
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1, 3, 6, 1],
    [1, 1, 1, 5, 1, 5, 1, 1],
    [1, 0, 5, 0, 0, 0, 4, 2],
    [1, 0, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
]
```

### Aturan permainan
1. HP awal **5**, maksimal **5**.
2. Jebakan (`^`) aktif **setiap kali diinjak**, mengurangi HP sebesar damage, dan tidak pernah hilang.
3. Ramuan (`+`) langsung dipakai saat diinjak: HP bertambah 3 (tidak melebihi maksimal), ramuan **hilang dari peta**.
4. **HP tidak boleh kurang dari 0** dan tidak boleh underflow (tipe HP adalah `u8`).
5. HP mencapai 0 → game over: `Kamu tewas setelah N langkah.`
6. Menginjak jebakan tetap dihitung sebagai langkah.
7. Aturan kunci, pintu, dinding, luar peta, dan `q` sama seperti Tahap 2.
8. Baris status: `HP: 3/5 | Langkah: 4 | Kunci: tidak`.
9. Jebakan menampilkan `Jebakan! HP -2.` Ramuan menampilkan `Ramuan! HP +N.` (N = pemulihan yang sebenarnya, bisa kurang dari 3 jika HP hampir penuh).

### Aturan kode tahap ini
- `struct Player` (posisi, HP, kunci, langkah) dan `struct Game` (peta + pemain).
- `impl Game` minimal: `new`, `draw(&self)`, dan satu method yang memproses perintah dengan `&mut self`.
- Method pemrosesan mengembalikan `enum Outcome` (minimal: bergerak, terhalang, menang, tewas, keluar). **`main` yang memutuskan lanjut atau berhenti** berdasarkan `match` atas `Outcome`.
- `main` hanya memanggil method `Game`. Dilarang menyentuh `game.player.hp` atau `game.map` langsung dari `main`.
- Tambahkan varian `Tile::Trap(u8)` dan `Tile::Potion(u8)` dengan data.
- `match` tanpa `_ =>` tetap berlaku untuk enum buatanmu.

### Contoh tampilan
```
########
#@..#K+#
###^#^##
#.^...DE
#.######
########
HP: 5/5 | Langkah: 0 | Kunci: tidak
```

### ✅ Kriteria selesai
- [ ] Rute tercepat yang selamat: **14 langkah**, HP akhir **2**
- [ ] Menuju ke kotak jebakan di baris 3 lalu bolak-balik: pemain tewas tanpa panic (uji underflow)
- [ ] Ramuan tidak bisa dipakai dua kali
- [ ] Ramuan saat HP penuh tidak membuat HP melebihi 5
- [ ] `main` tidak mengakses field `Game` secara langsung
- [ ] `cargo clippy` bersih

### 🧠 Uji diri
- Method mana yang butuh `&mut self`, dan kenapa `draw` tidak?
- Apa keuntungan `handle` mengembalikan `Outcome` daripada langsung memanggil `break` di dalamnya?

---

# 🎒 TAHAP 4: Inventori

**Tujuan belajar:** `Option` di dalam array, `.take()`, perpindahan kepemilikan nyata.

### Yang berubah dari Tahap 3
- Kunci dan ramuan **tidak lagi otomatis dipakai**. Keduanya masuk ke **tas berkapasitas 2**.
- Pintu **menghabiskan** kunci.
- Perintah baru, dan field `punya kunci` dihapus.

### Data yang disediakan
Sama seperti Tahap 3, kecuali kotak baris 4 kolom 1 sekarang `6` (ramuan kedua):

```rust
[
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1, 3, 6, 1],
    [1, 1, 1, 5, 1, 5, 1, 1],
    [1, 0, 5, 0, 0, 0, 4, 2],
    [1, 6, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
]
```

### Aturan permainan
1. Tas punya **2 slot**. Tiap slot kosong atau berisi satu barang.
2. Menginjak kunci/ramuan → barang masuk ke **slot kosong pertama**, kotak jadi lantai, pesan `Mengambil Kunci.` / `Mengambil Ramuan.`
3. Jika tas penuh → `Inventori penuh.` Pemain tetap berpindah ke kotak itu dan barangnya **tetap di peta**. Menginjaknya lagi nanti bisa mengambilnya jika sudah ada slot kosong.
4. Perintah `i` menampilkan tas: `Inventori: [1] Kunci  [2] (kosong)`. Tidak menambah langkah.
5. Perintah `p` meminum **ramuan pertama** di tas: HP +3 (maks 5), slot jadi kosong. Tidak menambah langkah. Jika tidak punya: `Tidak punya ramuan.`
6. Pintu: butuh **kunci di tas**. Jika ada: kunci **dikonsumsi** (slot kosong), kotak pintu jadi lantai, pemain pindah, pesan `Pintu terbuka.` Jika tidak: `Pintu terkunci.`
7. Jebakan, HP, kemenangan, kematian, dan `q` sama seperti Tahap 3.
8. Status menampilkan tas tiap putaran di baris tersendiri.

### Aturan kode tahap ini
- `enum Item { Key, Potion }`. Tas bertipe `[Option<Item>; 2]`.
- **Dilarang menyalin `Item`** (tidak ada `Copy`/`Clone`). Barang berpindah: tile → slot → hilang.
- Memindahkan barang dari slot memakai `.take()`.
- Status "punya kunci" **tidak boleh disimpan sebagai `bool`**. Ia selalu dihitung dari isi tas.
- Penelusuran slot memakai `for i in 0..2`, bukan iterator.
- `Command` bertambah varian untuk `i` dan `p`.

### Contoh tampilan
```
########
#@..#K+#
###^#^##
#.^...DE
#+######
########
HP: 5/5 | Langkah: 0
Inventori: [1] (kosong)  [2] (kosong)
```

### ✅ Kriteria selesai
- [ ] Rute tercepat yang selamat: **14 langkah**, HP akhir **2** (perintah `p` tidak dihitung langkah)
- [ ] Mengambil kunci + ramuan lalu menginjak ramuan ketiga → `Inventori penuh.` dan ramuan tetap di peta
- [ ] Setelah pintu dibuka, tas tidak lagi berisi kunci
- [ ] `p` tanpa ramuan tidak crash
- [ ] Tidak ada `.clone()` dan tidak ada `bool` untuk kunci
- [ ] `cargo clippy` bersih

### 🧠 Uji diri
- Apa yang dilakukan `.take()` pada slot, dan kenapa `let item = tas[0];` tidak bisa dipakai di sini?
- Dimana tepatnya kepemilikan sebuah `Item::Potion` berpindah selama hidupnya?

---

# 🧯 TAHAP 5: Input yang Waras (`Result` dan `?`)

**Tujuan belajar:** `Result`, operator `?`, enum error buatan sendiri, dan **kenapa** `?` memerlukan konversi error.

### Yang berubah dari Tahap 4
- Perintah `p` diganti `u`, yang **bertanya nomor slot** di baris berikutnya.
- Semua kegagalan jadi **nilai `Result`**, bukan `expect`, `panic`, atau `unwrap`.
- Pembacaan input perintah utama juga lewat `Result`.

### Aturan permainan
1. `u` → tampil `Pakai slot nomor berapa (1-2)?` lalu baca satu baris lagi.
2. Input slot bukan angka → error `NotANumber`.
3. Angka di luar 1-2 (termasuk 0 dan 3) → error `SlotOutOfRange`.
4. Slot kosong → error `SlotEmpty`.
5. Slot berisi kunci → error `NotUsable`.
6. Slot berisi ramuan → HP bertambah. Hasil sukses berisi **jumlah HP yang benar-benar pulih**, yang ditampilkan di pesan.
7. Input berakhir (EOF, `read_line` mengembalikan `Ok(0)`) → error `InputClosed` dan **game berakhir** dengan pesan.
8. Gagal membaca (error I/O) → error `InputFailed` dan **game berakhir** dengan pesan.
9. Error nomor 2-5 menampilkan pesan, **game lanjut**, langkah tidak berubah.

### Aturan kode tahap ini
- `enum GameError` dengan varian: `NotANumber`, `SlotOutOfRange`, `SlotEmpty`, `NotUsable`, `InputClosed`, `InputFailed`.
- Fungsi-fungsi berikut **semuanya mengembalikan `Result<..., GameError>`**:
  - `read_input()`
  - `parse_slot(...)`
  - satu method pemakaian slot pada `Game`
  - satu fungsi "alur" yang merangkai ketiganya
- Fungsi alur **wajib memakai `?`** pada ketiga pemanggilan itu.
- **Konversi error I/O dan error `parse` ke `GameError` ditulis manual dengan `match`.** Ini disengaja: kamu menulis sendiri apa yang nanti diotomatisasi `From`.
- `read_input()` mengembalikan `String` utuh **tanpa trim**. `trim` dilakukan oleh pemanggil.
- `main` **tidak boleh** memakai `unwrap` atau `expect` sama sekali.
- `main` menangani `Err` dengan `match` atas `GameError` (tanpa `_ =>`) dan memutuskan lanjut atau berhenti.
- Dilarang membuat `GameError` menampung `String`.

### Contoh tampilan
```
> u
Pakai slot nomor berapa (1-2)?
> abc
Error: itu bukan angka.
> u
Pakai slot nomor berapa (1-2)?
> 1
Error: slot 1 berisi kunci, tidak bisa dipakai.
> u
Pakai slot nomor berapa (1-2)?
> 2
Ramuan diminum. HP +3.
```

### ✅ Kriteria selesai
- [ ] Setiap skenario error 2-5 dan 7 teruji manual (EOF lewat `Ctrl+D`)
- [ ] Mengetik `-1`, `1.5`, spasi saja, baris kosong, dan angka sangat besar tidak membuat panic
- [ ] Tidak ada `unwrap`, `expect`, atau `panic!` di seluruh kode
- [ ] Fungsi alur memakai `?` tepat pada ketiga langkahnya
- [ ] Rute menang Tahap 4 tetap bisa dimainkan dengan `u` menggantikan `p`
- [ ] `cargo clippy` bersih

### 🧠 Uji diri
- Kenapa `?` tidak bisa dipakai langsung pada `read_line(...)` di fungsi yang mengembalikan `Result<_, GameError>`?
- Apa persis yang dilakukan `?` ketika menerima `Err`?
- Kenapa `read_input` lebih baik mengembalikan `String` utuh daripada hasil `trim`?

---

# 🔒 TAHAP 6: Konversi Otomatis dengan `From`

> **Syarat:** checklist "Trait dasar" selesai (`impl Trait for Tipe`, `Debug`, `Display`, `Clone`, `From`).

**Tujuan belajar:** `impl From`, `impl Display`, dan kenapa `?` bisa mengonversi error otomatis.

### Yang berubah dari Tahap 5
- Hapus semua `match` konversi manual.
- Pesan error tidak lagi ditulis di `match` di `main`, melainkan lewat `Display`.

### Aturan
1. `impl From<std::num::ParseIntError> for GameError` dan `impl From<std::io::Error> for GameError`.
2. `?` dipakai **langsung** pada `read_line(...)` dan `.parse::<usize>()`.
3. `impl std::fmt::Display for GameError` dan pesan error dicetak dengan `{}`.
4. Varian boleh menyimpan error asli (misalnya `NotANumber(ParseIntError)`).
5. Perilaku game **sama persis** dengan Tahap 5.
6. Aturan larangan lain tetap berlaku, kecuali `derive` dan trait yang kamu pelajari boleh dipakai.

### ✅ Kriteria selesai
- [ ] Tidak ada lagi `match` yang hanya berfungsi mengonversi error
- [ ] Perilaku Tahap 5 tidak berubah (semua skenario error masih lolos)
- [ ] Kamu bisa menjelaskan, tanpa melihat catatan, bagaimana `?` memanggil `From`

---

# 🔒 TAHAP 7: Ekosistem (`thiserror` dan `anyhow`)

> **Syarat:** Tahap 6 selesai dan kamu sudah belajar menambah dependency di `Cargo.toml`.

### Aturan
1. Ganti `Display` dan `From` manual dengan `#[derive(thiserror::Error)]`, `#[error("...")]`, dan `#[from]`.
2. Error yang **mengubah perilaku game** (`SlotEmpty`, `NotUsable`, dll.) tetap berupa `GameError`.
3. `main` mengembalikan `anyhow::Result<()>`. Error fatal (I/O gagal) naik ke `main` dengan `.context("...")`.
4. Perilaku game tetap sama seperti Tahap 6.

### ✅ Kriteria selesai
- [ ] Kode error lebih pendek dibanding Tahap 6 dengan perilaku sama
- [ ] Kamu bisa menjelaskan kapan memakai `thiserror` dan kapan `anyhow`
- [ ] Kamu paham baris `use anyhow::{Context, Result};` (alias `Result` dan trait `Context`)

---

## 🎯 Tantangan Bonus (opsional, setiap saat)
- [ ] Dua peta berbeda disimpan dalam array `[[[u8; 8]; 6]; 2]`, lanjut ke level berikutnya saat menang
- [ ] Skor akhir dari langkah dan HP tersisa
- [ ] Musuh diam di satu kotak yang mengurangi HP saat diinjak
- [ ] Setelah mempelajari testing: unit test untuk `next_pos`, `parse_command`, dan `parse_slot`
- [ ] Setelah mempelajari `Vec`: muat peta dari file teks

---

## 📊 Log Penyelesaian

| Tahap | Mulai | Selesai | Langkah tercepat | Yang paling membingungkan |
|:---:|---|---|:---:|---|
| 1 | | | /8 | |
| 2 | | | /12 | |
| 3 | | | /14 | |
| 4 | | | /14 | |
| 5 | | | | |
| 6 🔒 | | | | |
| 7 🔒 | | | | |

---

## 🤖 Template Minta Review ke AI

> Ini kode **Tahap [N]** proyek Labirin Terminal saya. Saya baru belajar sampai **[topik terakhir]**.
> Tolong review hanya terhadap: (1) aturan kode tahap ini, (2) kepemilikan data yang kurang tepat,
> (3) bagian yang tidak idiomatis. **Jangan menulis ulang solusi.** Tunjuk masalahnya dan beri satu petunjuk.
> Jika menyebut konsep di luar yang saya pelajari, beri satu kalimat saja dan tandai "parkir".

[tempel kode + hasil `cargo clippy`]
