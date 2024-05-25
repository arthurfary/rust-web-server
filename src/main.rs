#[macro_use]
extern crate nickel;

use nickel::Nickel;
use std::fs;

// readfile takes a string literal and returns a
// Result type with the ok being a String type
fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn parse_file_name(file_string: String) -> String {
    // If the input is just "/", return "index.html"
    if file_string == "/" {
        return String::from("index.html");
    }

    // Remove the leading '/' from the file_string
    let trimmed_string = &file_string[1..];

    // If the trimmed string ends with '/', append "index.html"
    if trimmed_string.ends_with('/') {
        format!("{}index.html", trimmed_string)
    }
    // If the trimmed string already ends with ".html", return it as is
    else if trimmed_string.ends_with(".html") {
        String::from(trimmed_string)
    }
    // Otherwise, append ".html" to the trimmed string
    else {
        format!("{}.html", trimmed_string)
    }
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |req, _res| {
            println!("{:?}", req.origin.uri);

            let file_path = req.origin.uri.to_string();
            let parsed_file_name = &parse_file_name(file_path);
            match read_file(parsed_file_name) {
                Ok(content) => content,
                Err(e) => format!("{parsed_file_name} not found: {e}"),
            }
        }
    });

    server.listen("localhost:6767").unwrap();
}
