# 🦀 Modul Pembelajaran Rust: Enum & Pattern Matching

**Fase ROADMAP:** Fase 1 — Inti Bahasa  
**Topik:** `enum`, pattern matching dengan `match`, `if let`, dan pola terkait  
**Tujuan:** memahami cara Rust memodelkan “salah satu dari beberapa kemungkinan” secara aman dan ekspresif.

---

## 1. Kenapa `enum` di Rust penting?

Di banyak bahasa, enum hanya daftar konstanta.

Contoh C:

```c
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
};
```

Di Rust, enum jauh lebih kuat.

Enum Rust adalah **sum type**:

> Sebuah tipe yang bisa memiliki beberapa varian, dan setiap varian boleh membawa data.

Contoh sederhana:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

Contoh enum yang membawa data:

```rust
enum ConfigValue {
    Int(i64),
    Text(String),
    Bool(bool),
}
```

Artinya:

- `ConfigValue` bisa berupa `Int`
- atau `Text`
- atau `Bool`

Tetapi tidak bisa semuanya sekaligus.

Ini sangat berguna untuk:

- parser konfigurasi
- command CLI
- state machine
- message/event system
- error handling
- protokol IPC
- UI state

---

## 2. Tiga bentuk varian enum

Rust memiliki tiga bentuk varian enum yang sering dipakai.

### 2.1 Unit variant

Varian tanpa data.

```rust
enum Command {
    Help,
    Version,
    Quit,
}
```

Cara membuat nilai:

```rust
let cmd = Command::Quit;
```

Mirip enum biasa di C/Go.

---

### 2.2 Tuple variant

Varian yang membawa data seperti tuple.

```rust
enum Message {
    Move(i32, i32),
    Write(String),
}
```

Cara membuat nilai:

```rust
let msg1 = Message::Move(10, 20);
let msg2 = Message::Write(String::from("hello"));
```

---

### 2.3 Struct variant

Varian yang membawa field bernama.

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

Cara membuat nilai:

```rust
let circle = Shape::Circle {
    radius: 2.0,
};

let rect = Shape::Rectangle {
    width: 10.0,
    height: 5.0,
};
```

---

## 3. Enum dipakai untuk memodelkan keadaan yang valid

Contoh kurang aman jika memakai string:

```rust
let position = "top";
```

Masalahnya:

```rust
let position = "tpo";
```

Compiler tidak tahu itu typo.

Dengan enum:

```rust
enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

let position = PanelPosition::Top;
```

Jika menulis:

```rust
let position = PanelPosition::Tpo;
```

compiler langsung error.

Ini salah satu prinsip penting di Rust:

> Make illegal states unrepresentable.

Buat state tidak valid menjadi tidak mungkin direpresentasikan.

---

## 4. `match`: pattern matching utama di Rust

`match` digunakan untuk menangani semua kemungkinan varian enum.

Contoh:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn describe(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    }
}
```

Cara pakai:

```rust
let d = Direction::North;
println!("{}", describe(d));
```

---

## 5. `match` adalah expression

Ini nyambung dengan konsep expression vs statement.

Karena `match` adalah expression, ia bisa menghasilkan nilai.

```rust
let direction = Direction::North;

let label = match direction {
    Direction::North => "north",
    Direction::South => "south",
    Direction::East => "east",
    Direction::West => "west",
};
```

Semua arm harus menghasilkan tipe yang sama.

Contoh error:

```rust
let label = match direction {
    Direction::North => "north",
    Direction::South => 10,
    _ => "other",
};
```

Ini error karena satu arm menghasilkan `&str`, arm lain menghasilkan integer.

---

## 6. `match` harus exhaustive

Aturan penting:

> `match` harus menangani semua kemungkinan varian enum.

Contoh tidak lengkap:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn describe(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
    }
}
```

Compiler error karena `East` dan `West` belum ditangani.

Bisa ditutup dengan wildcard:

```rust
fn describe(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
        _ => "other",
    }
}
```

Tapi hati-hati.

Jika nanti enum bertambah:

```rust
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
}
```

maka `match` tanpa `_` akan error di semua tempat yang belum menangani varian baru.

Ini sering justru bagus.

Dengan `_`, compiler tidak memaksa Anda menangani varian baru secara eksplisit.

---

## 7. Pattern matching untuk mengambil data dari enum

Enum menjadi sangat kuat saat variannya membawa data.

```rust
enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
}

fn process(message: Message) {
    match message {
        Message::Quit => {
            println!("quit");
        }
        Message::Move(x, y) => {
            println!("move to {x}, {y}");
        }
        Message::Write(text) => {
            println!("write: {text}");
        }
    }
}
```

Penjelasan:

```rust
Message::Move(x, y)
```

mengikat nilai pertama ke `x`, nilai kedua ke `y`.

```rust
Message::Write(text)
```

mengikat isi `String` ke variabel `text`.

---

## 8. Matching pada struct variant

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => {
            std::f64::consts::PI * radius * radius
        }
        Shape::Rectangle { width, height } => {
            width * height
        }
    }
}
```

Cara pakai:

```rust
let c = Shape::Circle { radius: 2.0 };
let r = Shape::Rectangle {
    width: 3.0,
    height: 4.0,
};

println!("{}", area(c));
println!("{}", area(r));
```

---

## 9. Mengabaikan nilai dengan `_`

Gunakan `_` jika tidak peduli dengan nilai tertentu.

```rust
enum Message {
    Move(i32, i32),
    Write(String),
}

fn only_y(message: Message) {
    match message {
        Message::Move(_, y) => {
            println!("y = {y}");
        }
        Message::Write(_) => {
            println!("not a move");
        }
    }
}
```

`_` tidak mengikat nilai.

---

## 10. Mengabaikan field dengan `..`

Untuk struct variant, gunakan `..` untuk mengabaikan field lain.

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn width_or_zero(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle { width, .. } => width,
        _ => 0.0,
    }
}
```

Artinya:

> Ambil `width`, abaikan field lain.

---

## 11. Pola alternatif dengan `|`

Anda bisa menggabungkan beberapa pattern.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn is_vertical(direction: Direction) -> bool {
    match direction {
        Direction::North | Direction::South => true,
        Direction::East | Direction::West => false,
    }
}
```

`|` artinya “atau”.

---

## 12. Match dengan literal

Pattern matching tidak hanya untuk enum.

```rust
fn describe_code(code: u32) -> &'static str {
    match code {
        200 => "ok",
        404 => "not found",
        500 => "server error",
        _ => "other",
    }
}
```

Ini mirip `switch`, tetapi lebih kuat.

---

## 13. Match dengan range

```rust
fn category(code: u32) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirection",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}
```

Nanti berguna untuk HTTP status code di Fase 3.

---

## 14. Match guard

Anda bisa menambahkan kondisi `if` pada pattern.

```rust
enum ConfigValue {
    Int(i64),
    Text(String),
    Bool(bool),
}

fn describe(value: ConfigValue) -> String {
    match value {
        ConfigValue::Int(n) if n > 0 => format!("positive int: {n}"),
        ConfigValue::Int(n) => format!("int: {n}"),
        ConfigValue::Text(s) => format!("text: {s}"),
        ConfigValue::Bool(b) => format!("bool: {b}"),
    }
}
```

Contoh pattern:

```rust
ConfigValue::Int(n) if n > 0
```

Artinya:

> Match varian `Int`, lalu pastikan nilainya lebih dari 0.

---

## 15. `if let`: match ringkas untuk satu kasus

Kadang Anda hanya peduli satu varian.

Daripada menulis:

```rust
match message {
    Message::Move(x, y) => {
        println!("move to {x}, {y}");
    }
    _ => {}
}
```

Anda bisa menulis:

```rust
if let Message::Move(x, y) = message {
    println!("move to {x}, {y}");
}
```

Ini lebih ringkas.

---

## 16. `if let` dengan `else`

```rust
enum ConfigValue {
    Int(i64),
    Text(String),
    Bool(bool),
}

fn process(value: ConfigValue) {
    if let ConfigValue::Text(text) = value {
        println!("got text: {text}");
    } else {
        println!("not text");
    }
}
```

Gunakan `if let` jika:

- hanya peduli satu pattern
- kasus lain tidak penting
- ingin kode lebih pendek

Gunakan `match` jika:

- ada beberapa varian penting
- ingin exhaustive
- ingin compiler memaksa semua kasus ditangani

---

## 17. `while let`

`while let` menjalankan loop selama pattern masih cocok.

Contoh dengan `Option`:

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("top = {top}");
}
```

Ini setara dengan loop yang terus memeriksa apakah `pop()` menghasilkan `Some`.

---

## 18. Enum bisa punya method dengan `impl`

Enum juga bisa memiliki method.

```rust
enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl PanelPosition {
    fn as_str(&self) -> &'static str {
        match self {
            PanelPosition::Top => "top",
            PanelPosition::Bottom => "bottom",
            PanelPosition::Left => "left",
            PanelPosition::Right => "right",
        }
    }
}
```

Cara pakai:

```rust
let pos = PanelPosition::Top;
println!("{}", pos.as_str());
```

---

## 19. Contoh enum dengan method yang lebih nyata

```rust
enum ConfigValue {
    Int(i64),
    Text(String),
    Bool(bool),
}

impl ConfigValue {
    fn type_name(&self) -> &'static str {
        match self {
            ConfigValue::Int(_) => "int",
            ConfigValue::Text(_) => "text",
            ConfigValue::Bool(_) => "bool",
        }
    }
}
```

Cara pakai:

```rust
let value = ConfigValue::Int(42);
println!("{}", value.type_name());
```

---

## 20. Borrowing saat match

Ini penting karena nyambung dengan ownership dan borrowing.

Jika Anda match pada nilai owned:

```rust
match command {
    Command::SetWallpaper(path) => {
        println!("{path}");
    }
    _ => {}
}
```

`path` bisa menjadi owned `String`.

Artinya nilai bisa dipindahkan keluar dari `command`.

Jika Anda match pada referensi:

```rust
match &command {
    Command::SetWallpaper(path) => {
        println!("{path}");
    }
    _ => {}
}
```

maka `path` biasanya menjadi `&String`.

Ini penting jika `command` masih ingin dipakai setelah match.

Contoh:

```rust
#[derive(Debug)]
enum Command {
    Help,
    SetWallpaper(String),
}

fn main() {
    let command = Command::SetWallpaper(String::from("wall.png"));

    match &command {
        Command::SetWallpaper(path) => {
            println!("path: {path}");
        }
        _ => {}
    }

    // command masih bisa dipakai karena tadi hanya dipinjam
    println!("{command:?}");
}
```

Jika match memakai nilai owned:

```rust
match command {
    Command::SetWallpaper(path) => {
        println!("path: {path}");
    }
    _ => {}
}
```

maka `command` bisa sudah dipindahkan dan tidak selalu bisa dipakai lagi.

---

## 21. Enum terkenal di Rust: `Option<T>`

`Option<T>` adalah enum bawaan Rust.

Definisi konseptualnya:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Ini digunakan untuk nilai yang mungkin ada, mungkin tidak.

Contoh:

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("budi"))
    } else {
        None
    }
}
```

Match:

```rust
match find_user(1) {
    Some(name) => println!("user: {name}"),
    None => println!("user not found"),
}
```

Dengan `if let`:

```rust
if let Some(name) = find_user(1) {
    println!("user: {name}");
}
```

Ini akan sangat penting di Fase 1 dan Fase 2.

---

## 22. Enum terkenal di Rust: `Result<T, E>`

`Result<T, E>` juga enum.

Definisi konseptualnya:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Ini digunakan untuk operasi yang bisa gagal.

Contoh:

```rust
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
```

Match:

```rust
match read_file("config.toml") {
    Ok(content) => println!("content: {content}"),
    Err(e) => println!("error: {e}"),
}
```

Dengan `if let`:

```rust
if let Ok(content) = read_file("config.toml") {
    println!("content: {content}");
}
```

Nanti Anda akan sering memakai `Result` bersama operator `?`.

---

## 23. Macro `matches!`

Kadang Anda hanya ingin mengecek apakah sebuah nilai cocok dengan pattern.

```rust
enum Command {
    Help,
    Quit,
    SetWallpaper(String),
}

fn is_quit(command: &Command) -> bool {
    matches!(command, Command::Quit)
}
```

`matches!` menghasilkan `bool`.

Contoh lain:

```rust
let x = 5;

let is_small = matches!(x, 0..=10);
```

---

## 24. `let else`

Ini pola yang lebih baru dan berguna untuk early return.

Contoh:

```rust
fn process(value: Option<String>) {
    let Some(name) = value else {
        println!("no name");
        return;
    };

    println!("name: {name}");
}
```

Artinya:

- jika `value` adalah `Some(name)`, lanjutkan
- jika tidak, jalankan blok `else`
- blok `else` harus keluar dari scope, misalnya dengan `return`, `break`, `continue`, atau `panic!`

Ini tidak wajib di Fase 1, tapi berguna nanti.

---

## 25. Perbedaan dengan C

### C enum hanya integer

```c
enum Color {
    RED,
    GREEN,
    BLUE
};
```

Rust:

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

Varian Rust adalah nilai bertipe `Color`, bukan integer biasa.

---

### C butuh union manual untuk varian berdata

Di C, sum type biasanya dibuat manual:

```c
enum ValueKind {
    VALUE_INT,
    VALUE_TEXT,
    VALUE_BOOL
};

struct Value {
    enum ValueKind kind;
    union {
        int i;
        char *text;
        bool b;
    } data;
};
```

Di Rust:

```rust
enum Value {
    Int(i64),
    Text(String),
    Bool(bool),
}
```

Compiler membantu memastikan varian ditangani dengan benar.

---

### C switch tidak wajib exhaustive

Di C, Anda bisa lupa menangani satu case.

Di Rust, `match` harus exhaustive.

---

## 26. Perbedaan dengan Go

Go tidak punya enum sebagai sum type.

Biasanya:

```go
type Direction int

const (
    North Direction = iota
    South
    East
    West
)
```

Untuk varian berdata, Go sering memakai interface:

```go
type Shape interface{}
```

Lalu type switch:

```go
switch s := shape.(type) {
case Circle:
    // ...
case Rectangle:
    // ...
}
```

Rust lebih eksplisit:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

Lalu:

```rust
match shape {
    Shape::Circle { radius } => {}
    Shape::Rectangle { width, height } => {}
}
```

---

## 27. Perbedaan dengan Java

Java enum bisa punya method:

```java
enum Direction {
    NORTH,
    SOUTH;
}
```

Tapi enum Java tidak sefleksibel Rust enum untuk membawa data berbeda per varian.

Rust:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

Ini satu tipe utuh, dan compiler bisa memeriksa semua varian.

---

## 28. Enum dan memory layout

Untuk Anda yang dari C, mungkin bertanya:

> Berapa ukuran enum?

Secara konsep, enum Rust seperti tagged union:

- ada discriminant/tag untuk tahu varian aktif
- ada ruang untuk data varian terbesar

Contoh:

```rust
enum Value {
    Bool(bool),
    Int(i64),
    Text(String),
}
```

Ukuran `Value` harus cukup untuk varian terbesar, plus discriminant dan padding.

Compiler bisa melakukan optimasi tertentu.

Untuk Fase 1, tidak perlu menghafal layout memori.

Yang penting:

> Enum memungkinkan satu tipe memiliki beberapa varian, dan compiler memastikan Anda menangani varian secara benar.

Jika butuh layout kompatibel dengan C, ada atribut seperti:

```rust
#[repr(C)]
```

atau:

```rust
#[repr(u8)]
```

Tapi itu topik lanjutan.

---

## 29. Hubungan dengan ROADMAP Fase 1

Di ROADMAP, Fase 1 punya mini-proyek:

- CLI pemilih wallpaper
- parser konfigurasi TOML

Enum sangat relevan.

Untuk CLI:

```rust
enum CliCommand {
    Help,
    List,
    Set(String),
    Random,
}
```

Untuk konfigurasi:

```rust
enum ConfigValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}
```

Untuk posisi panel:

```rust
enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}
```

Untuk error parser:

```rust
enum ParseError {
    MissingField(String),
    InvalidValue(String),
    Io(String),
}
```

Nanti di Fase 2 dan Fase 3, enum akan muncul lagi untuk:

- `Option<T>`
- `Result<T, E>`
- message daemon
- event loop
- IPC
- state machine
- command/event pattern

---

## 30. Pola desain: hindari state tidak valid

Kurang baik:

```rust
struct ConfigValue {
    int_value: i64,
    text_value: String,
    bool_value: bool,
}
```

State tidak valid bisa terjadi:

```rust
ConfigValue {
    int_value: 10,
    text_value: String::from("hello"),
    bool_value: true,
}
```

Lebih baik:

```rust
enum ConfigValue {
    Int(i64),
    Text(String),
    Bool(bool),
}
```

Dengan enum, nilai hanya bisa menjadi salah satu varian.

---

## 31. Contoh lengkap sederhana

```rust
#[derive(Debug)]
enum CliCommand {
    Help,
    Version,
    SetWallpaper(String),
    SetBrightness(u32),
}

impl CliCommand {
    fn describe(&self) -> &'static str {
        match self {
            CliCommand::Help => "show help",
            CliCommand::Version => "show version",
            CliCommand::SetWallpaper(_) => "set wallpaper",
            CliCommand::SetBrightness(_) => "set brightness",
        }
    }
}

fn run(command: CliCommand) {
    match command {
        CliCommand::Help => {
            println!("help");
        }
        CliCommand::Version => {
            println!("version");
        }
        CliCommand::SetWallpaper(path) => {
            println!("setting wallpaper: {path}");
        }
        CliCommand::SetBrightness(level) => {
            println!("setting brightness: {level}");
        }
    }
}

fn main() {
    let cmd = CliCommand::SetWallpaper(String::from("wall.png"));

    println!("{}", cmd.describe());
    run(cmd);
}
```

---

## 32. Ringkasan

### `enum`

Digunakan untuk membuat tipe dengan beberapa varian.

```rust
enum Command {
    Help,
    SetWallpaper(String),
}
```

Varian bisa:

- tanpa data
- membawa tuple
- membawa field bernama

---

### `match`

Digunakan untuk menangani semua varian.

```rust
match command {
    Command::Help => {}
    Command::SetWallpaper(path) => {}
}
```

Sifat penting:

- expression
- exhaustive
- bisa destructuring
- bisa menghasilkan nilai

---

### `if let`

Digunakan jika hanya peduli satu pattern.

```rust
if let Command::SetWallpaper(path) = command {
    println!("{path}");
}
```

Setara dengan:

```rust
match command {
    Command::SetWallpaper(path) => {
        println!("{path}");
    }
    _ => {}
}
```

---

### `while let`

Loop selama pattern cocok.

```rust
while let Some(value) = stack.pop() {
    println!("{value}");
}
```

---

### `Option<T>`

Enum untuk nilai yang mungkin ada atau tidak.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

### `Result<T, E>`

Enum untuk operasi yang bisa berhasil atau gagal.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

## 33. Common pitfalls

### 1. Menggunakan `_` terlalu cepat

```rust
match command {
    Command::Help => {}
    _ => {}
}
```

Ini valid, tapi jika nanti enum bertambah varian, compiler tidak memaksa Anda menangani varian baru.

Jika ingin aman, tangani semua varian secara eksplisit.

---

### 2. Mengira `if let` exhaustive

```rust
if let Command::Help = command {
    println!("help");
}
```

Ini tidak memaksa Anda menangani varian lain.

Jika semua varian penting, gunakan `match`.

---

### 3. Lupa bahwa matching by value bisa memindahkan data

```rust
match command {
    Command::SetWallpaper(path) => {
        println!("{path}");
    }
    _ => {}
}
```

Jika `command` owned, `path` bisa menjadi owned `String`.

Jika `command` masih ingin dipakai, match pada referensi:

```rust
match &command {
    Command::SetWallpaper(path) => {
        println!("{path}");
    }
    _ => {}
}
```

---

### 4. Mengira varian enum adalah tipe terpisah

Ini bukan tipe:

```rust
Command::Help
```

Ini adalah varian dari tipe:

```rust
Command
```

Jadi Anda biasanya menulis fungsi yang menerima `Command`, bukan `Command::Help`.

---

## 34. Prompt berpikir kritis

Coba jawab sendiri dulu:

1. Jika Anda menambah varian baru ke enum, apa yang terjadi pada semua `match` tanpa `_`?

2. Kenapa ini bisa dianggap fitur, bukan gangguan?

3. Kapan Anda memilih `if let` daripada `match`?

4. Apa yang Anda korbankan saat memakai `if let`?

5. Untuk parser TOML, kenapa enum berikut:

   ```rust
   enum ConfigValue {
       Int(i64),
       Text(String),
       Bool(bool),
   }
   ```

   lebih aman daripada struct berikut:

   ```rust
   struct ConfigValue {
       int_value: i64,
       text_value: String,
       bool_value: bool,
   }
   ```

6. Jika Anda match pada `&command`, kenapa data yang Anda dapatkan biasanya berupa reference, bukan owned value?

---

## 35. Cheat sheet cepat

```rust
// Enum sederhana
enum Direction {
    North,
    South,
}

// Enum tuple
enum Message {
    Move(i32, i32),
    Write(String),
}

// Enum struct-like
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

// Match
match direction {
    Direction::North => {}
    Direction::South => {}
}

// Match dengan data
match message {
    Message::Move(x, y) => {}
    Message::Write(text) => {}
}

// Match wildcard
match message {
    Message::Move(_, y) => {}
    _ => {}
}

// Match alternatif
match direction {
    Direction::North | Direction::South => {}
}

// if let
if let Message::Move(x, y) = message {
    println!("{x}, {y}");
}

// if let else
if let Message::Write(text) = message {
    println!("{text}");
} else {
    println!("not write");
}

// while let
while let Some(item) = vec.pop() {
    println!("{item}");
}

// matches!
let is_move = matches!(message, Message::Move(_, _));
```

---

## Referensi

### APA

Klabnik, S., & Nichols, C. (2023). *The Rust programming language* (Chapter 6: Enums and pattern matching; Chapter 18: Patterns and matching). No Starch Press. https://doc.rust-lang.org/book/

Rust contributors. (2024). *Rust by example: Enums*. https://doc.rust-lang.org/rust-by-example/

Google Engineering. (n.d.). *Comprehensive Rust: Enums and pattern matching*. https://google.github.io/comprehensive-rust/

### MLA

Klabnik, Steve, and Carol Nichols. *The Rust Programming Language*. No Starch Press, 2023. https://doc.rust-lang.org/book/.

Rust contributors. “Enums.” *Rust by Example*, https://doc.rust-lang.org/rust-by-example/.

Google Engineering. “Enums and Pattern Matching.” *Comprehensive Rust*, https://google.github.io/comprehensive-rust/.
