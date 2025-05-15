pub fn run() {
    println!("Hari 2: Control Flow & Functions\n");

    percabangan_if(12);
    percabangan_match(3);
    
    println!("\n🔁 Looping:");
    loop_example();
    while_example();
    for_example();

    println!("\n🔧 Fungsi dan Parameter:");
    greet("Rustacean");
    let result = tambah(5, 7);
    println!("Hasil penjumlahan 5 + 7 = {}", result);

    println!("\n📦 Scope dan Shadowing:");
    scope_and_shadowing();
}

// 1. Percabangan if
fn percabangan_if(x: i32) {
    println!("Percabangan if:");
    if x > 10 {
        println!("Nilai {} lebih besar dari 10", x);
    } else if x == 10 {
        println!("Nilai {} sama dengan 10", x);
    } else {
        println!("Nilai {} lebih kecil dari 10", x);
    }
}

// 1. Percabangan match
fn percabangan_match(x: i32) {
    println!("\nPercabangan match:");
    match x {
        1 => println!("Satu"),
        2 | 3 => println!("Dua atau Tiga"),
        4..=10 => println!("Antara 4 sampai 10"),
        _ => println!("Nilai lainnya"),
    }
}

// 2. Loop: loop
fn loop_example() {
    println!("Contoh loop:");
    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }
        println!("  loop ke-{}", counter);
        counter += 1;
    }
}

// 2. Loop: while
fn while_example() {
    println!("\nContoh while:");
    let mut number = 0;
    while number < 3 {
        println!("  while ke-{}", number);
        number += 1;
    }
}

// 2. Loop: for
fn for_example() {
    println!("\nContoh for:");
    for i in 1..=3 {
        println!("  for ke-{}", i);
    }
}

// 3. Function dengan parameter
fn greet(name: &str) {
    println!("Halo, {}! Selamat belajar Rust!", name);
}

// 3. Function dengan return
fn tambah(a: i32, b: i32) -> i32 {
    a + b // tidak perlu return jika ini baris terakhir
}

// 4. Scope dan Shadowing
fn scope_and_shadowing() {
    let x = 5;
    println!("Nilai awal x = {}", x);

    {
        let x = x + 10;
        println!("Nilai x di dalam block = {}", x);
    }

    println!("Nilai x di luar block = {}", x); // tetap 5
}
