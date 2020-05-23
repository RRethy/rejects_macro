extern crate rejects_macro;

use rejects_macro::rejects;

rejects! { r"\d\d\d" }

fn main() {
    let re = foobarbaz();
    println!("{}", re.find_end("123ds"));
    println!("Hello, world!");
}
