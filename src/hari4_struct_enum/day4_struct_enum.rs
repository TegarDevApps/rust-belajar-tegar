pub fn run() {
    println!("Hari 4: Struct, Enum, dan Pattern Matching\n");

    // 1. Struct dan impl
    let user = User::new("Tegar", 25);
    user.introduce();

    // 2. Enum dan match
    let status = Status::InProgress;
    print_status(status);

    // 3. Option
    println!("\n🔍 Option:");
    let angka = Some(7);
    let kosong: Option<i32> = None;
    print_option(angka);
    print_option(kosong);

    // 4. Result
    println!("\n✅ Result:");
    match bagi(10, 2) {
        Ok(hasil) => println!("10 dibagi 2 = {}", hasil),
        Err(e) => println!("Error: {}", e),
    }

    match bagi(5, 0) {
        Ok(hasil) => println!("5 dibagi 0 = {}", hasil),
        Err(e) => println!("Error: {}", e),
    }
}

// 1. Struct dan impl
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn introduce(&self) {
        println!("👤 Halo, saya {} dan saya berumur {} tahun.", self.name, self.age);
    }
}

// 2. Enum dan match
enum Status {
    Todo,
    InProgress,
    Done,
}

fn print_status(status: Status) {
    println!("\n📌 Status tugas:");
    match status {
        Status::Todo => println!("Belum dikerjakan"),
        Status::InProgress => println!("Sedang dikerjakan"),
        Status::Done => println!("Selesai"),
    }
}

// 3. Option
fn print_option(data: Option<i32>) {
    match data {
        Some(n) => println!("Angka ditemukan: {}", n),
        None => println!("Tidak ada angka"),
    }
}

// 4. Result
fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Tidak bisa dibagi dengan nol"))
    } else {
        Ok(a / b)
    }
}
