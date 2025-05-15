// src/main.rs

mod hari1_pengantar_rust {
    pub mod day1_basic;
}

mod hari2_control_flow {
    pub mod day2_control;
}

fn main() {
    println!("Mulai Program Rust...");
    hari1_pengantar_rust::day1_basic::run();
}
