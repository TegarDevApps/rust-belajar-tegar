// src/main.rs

mod hari1_pengantar_rust {
    pub mod day1_basic;
}

mod hari2_control_flow {
    pub mod day2_control;
}

mod hari3_ownership {
    pub mod day3_ownership;
}

fn main() {
    println!("Mulai Program Rust...");
    hari3_ownership::day3_ownership::run();
}
