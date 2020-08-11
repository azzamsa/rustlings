// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// rust macro must defined fist
fn main() {
    my_macro!();
}
