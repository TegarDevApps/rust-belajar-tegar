pub fn run() {
    println!("Hari 3: Ownership, Borrowing & Lifetimes\n");

    ownership_example();
    borrowing_example();
    mutable_borrowing_example();
    string_vs_str_example();
    lifetime_basic();
}

// 1. Ownership Rules
fn ownership_example() {
    println!("📦 Ownership:");

    let s1 = String::from("Hello");
    let s2 = s1; // ownership berpindah ke s2

    // println!("{}", s1); // ❌ error: s1 sudah tidak valid
    println!("s2: {}", s2);
}

// 2. Borrowing: immutable reference
fn borrowing_example() {
    println!("\n📌 Borrowing (immutable):");

    let s = String::from("Rustacean");
    print_length(&s); // dipinjam, bukan dipindahkan
    println!("s tetap bisa digunakan: {}", s);
}

fn print_length(s: &String) {
    println!("Panjang string: {}", s.len());
}

// 2. Borrowing: mutable reference
fn mutable_borrowing_example() {
    println!("\n✏️ Borrowing (mutable):");

    let mut s = String::from("Halo");
    append_text(&mut s);
    println!("Setelah dimodifikasi: {}", s);
}

fn append_text(s: &mut String) {
    s.push_str(", dunia!");
}

// 3. String vs &str
fn string_vs_str_example() {
    println!("\n🧵 String vs &str:");

    let s_literal: &str = "Hello"; // string slice, immutable
    let s_dynamic: String = String::from("World"); // heap-allocated

    println!("Literal (&str): {}", s_literal);
    println!("Dinamis (String): {}", s_dynamic);
}

// 4. Lifetime (konsep dasar)
fn lifetime_basic() {
    println!("\n⏳ Lifetime (konsep dasar):");

    let r;
    {
        let x = 42;
        r = x;
        println!("Nilai r: {}", r); // ini aman karena x belum habis scope-nya
    }
}
