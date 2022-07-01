use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My names is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
    hello_macro = { path = "../hello_macro" }
    hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
}
