// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

mod my_macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    use ::my_macro;
    my_macro!();
}
