pub fn run () {
    println!("-- Belajar Dasar Rust --");

    // Variabel immutable
    let x = 5;
    println!("Nilai x (immutable): {}", x);

    // Variabel mutable
    let mut y = 10;
    println!("Nilai awal y (mutable): {}", y);

    y = 20;
    println!("Nilai y setelah diubah: {}", y);

    // Tipe data
    let a: i32 = 100;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';
    let e: &str = "Hello, Rust!";

    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
}