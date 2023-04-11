use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let slashes = String::from("    ///////////////////////////////////////////////////////////////////");

    let slashes_len = slashes.len();

    let mut space_start = String::from("");

    while (61 - (space_start.len() + input.len())) > space_start.len() {
        space_start.push_str(" ")
    }

    let space_len = space_start.len();
    
    let space_end_len = slashes_len - 10 - space_len - input.len();
    
    let mut space_end = String::from("");

    for _ in 0..space_end_len {
        space_end.push_str(" ")
    }

    let output = format!(
        "{}\n{}{}{}{}{}{}\n{}",
        slashes,
        "    ",
        "///",
        space_start,
        input.to_uppercase(),
        space_end,
        "///",
        slashes
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}
