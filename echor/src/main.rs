use clap::Command;

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Ryoei Shimaura <tinytortoise.dev@gmail.com>")
        .about("Rust version of `echo`")
        .get_matches();
}
