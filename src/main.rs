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

mod hari4_struct_enum {
    pub mod day4_struct_enum;
}

mod hari5_collections_error {
    pub mod day5_collections_error;
}

fn main() {
    println!("Mulai Program Rust...");
    hari5_collections_error::day5_collections_error::run();
}
