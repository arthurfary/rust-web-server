# Rust Web Server

This is a very bare-bones Web Server I created in Rust for the goal of
learning it.

---

## How to download

To use you will need to clone this repo. 
- Clone the repo with `git clone https://github.com/arthurfary/rust-web-server`.
- Build it for you system using [Cargo]{https://doc.rust-lang.org/cargo/getting-started/installation.html} 
by running `cargo build -r`
- Your executable will be on the `target/release/` folder, named `rust-web-server`

## How to use

Simply run the executable in the root folder of your project and the server will serve any **.html** files.

If a name isn't provided, the server will look for a `index.html`, inside the requested folder.

## Examples

The server will display files put in the root directory (or the directory where the server is run) and its available sub-directories.

In the following examples, the server is run on the root directory.

| The url in the browser        | The file displayed       |
|-------------------------------|--------------------------|
| localhost:6767                | root/index.html          |
| localhost:6767/about          | root/about.html          |
| localhost:6767/about.html     | root/about.html          |
| localhost:6767/userinfo/      | root/userinfo/index.html |
| localhost:6767/userinfo/table | root/userinfo/table.html |

---
## How to contribute

Simply clone the project and start coding! PR's are welcomed :)
