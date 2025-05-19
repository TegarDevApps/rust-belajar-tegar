# Belajar Rust - Hari 5

## 🗓 Tanggal: Senin, 19 Mei 2025
---

## Topik: Collections & Error Handling

### Materi yang Dipelajari

1. **Vec<T>**
   - Vektor dinamis, bisa menambah dan mengakses elemen.
   - Akses elemen bisa pakai `.get(index)` yang mengembalikan `Option`.

2. **HashMap<K, V>**
   - Koleksi kunci-nilai, seperti dictionary.
   - Mendukung pengecekan data menggunakan `.get(kunci)`.

3. **Error Handling**
   - Menggunakan `Result<T, E>` untuk kesalahan yang bisa ditangani (recoverable).
   - Operator `?` digunakan untuk propagate error secara ringkas.

4. **panic! vs Recoverable Error**
   - `panic!` menghentikan program (unrecoverable).
   - `Result` memberikan kontrol untuk menangani kesalahan tanpa crash.

---

## Praktik Hari Ini

File: [`src/hari5_collections_error/day5_collections_error.rs`](./src/hari5_collections_error/day5_collections_error.rs)  
Dipanggil dari: [`src/main.rs`](./src/main.rs)

---

### Contoh Output Terminal

![Output Hari 5](./assets/day5.jpeg)