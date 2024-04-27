// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

macro_rules! my_macro {
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
