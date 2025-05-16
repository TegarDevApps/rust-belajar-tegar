# Belajar Rust - Hari 3

## 🗓 Tanggal: 16 Mei 2025
---

## Topik: Ownership, Borrowing & Lifetimes

### Materi yang Dipelajari

1. **Ownership Rules**
   - Setiap value di Rust memiliki pemilik (owner).
   - Saat value dipindah ke variabel lain, owner lama tidak bisa digunakan lagi.

2. **Borrowing (`&`, `&mut`)**
   - Meminjam variabel tanpa memindahkan ownership.
   - `&` → pinjaman tidak bisa diubah (immutable).
   - `&mut` → pinjaman yang bisa dimodifikasi (mutable).

3. **`String` vs `&str`**
   - `String` adalah tipe data dinamis yang bisa diubah.
   - `&str` adalah slice string yang immutable.

4. **Lifetime (Konsep Dasar)**
   - Lifetime menunjukkan seberapa lama referensi valid.
   - Berguna untuk memastikan referensi tidak mengarah ke data yang sudah hangus.

---

## Praktik Hari Ini

File: [`src/hari3_ownership/day3_ownership.rs`](./src/hari3_ownership/day3_ownership.rs)  
Dipanggil dari: [`src/main.rs`](./src/main.rs)

### Contoh Output Terminal:

```bash
Hari 3: Ownership, Borrowing & Lifetimes

📦 Ownership:
s2: Hello

📌 Borrowing (immutable):
Panjang string: 9
s tetap bisa digunakan: Rustacean

✏️ Borrowing (mutable):
Setelah dimodifikasi: Halo, dunia!

🧵 String vs &str:
Literal (&str): Hello
Dinamis (String): World

⏳ Lifetime (konsep dasar):
Nilai r: 42
