use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

pub fn run() {
    println!("Hari 5: Collections & Error Handling\n");

    // ======== VEC =========
    let mut angka: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    angka.push(4);
    angka.push(5);

    println!("Vektor angka:");
    for a in &angka {
        println!("- {}", a);
    }

    // Mengakses elemen dengan match
    match angka.get(10) {
        Some(val) => println!("Elemen ke-10: {}", val),
        None => println!("Tidak ada elemen ke-10"),
    }

    // ======== HASHMAP =========
    let mut nilai_siswa: HashMap<String, u32> = HashMap::new();
    nilai_siswa.insert("Tegar".to_string(), 80);
    nilai_siswa.insert("Budi".to_string(), 90);

    println!("\nNilai siswa:");
    for (nama, nilai) in &nilai_siswa {
        println!("{}: {}", nama, nilai);
    }

    // Cek nilai
    let nama = "Tegar";
    match nilai_siswa.get(nama) {
        Some(nilai) => println!("Nilai {} adalah {}", nama, nilai),
        None => println!("{} tidak ditemukan", nama),
    }

    // ======== Result dan ? =========
    match baca_file("src/hari5_collections_error/sample.txt") {
        Ok(data) => println!("\n📖 Isi file:\n{}", data),
        Err(e) => println!("\n❌ Gagal membaca file: {}", e),
    }

    // ======== Panic vs Recoverable Error =========
    println!("\nPanic vs Recoverable:");
    match bagi(10, 2) {
        Ok(hasil) => println!("10 / 2 = {}", hasil),
        Err(e) => println!("Error: {}", e),
    }

    // Ini akan panic
    // panic!("Terjadi kesalahan fatal!");
}

// Fungsi baca file dengan error handling Result dan ?
fn baca_file(nama_file: &str) -> Result<String, io::Error> {
    let mut file = File::open(nama_file)?; // gunakan `?` untuk propagate error
    let mut isi = String::new();
    file.read_to_string(&mut isi)?;
    Ok(isi)
}

// Fungsi dengan recoverable error
fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Pembagian dengan nol tidak diizinkan"))
    } else {
        Ok(a / b)
    }
}
