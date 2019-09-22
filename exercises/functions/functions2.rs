// functions2.rs
// Make me compile! Scroll down for hints :)

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

// Rust requires that all parts of a function's signature have type annotations,
// but `call_me` is missing the type annotation of `num`.
