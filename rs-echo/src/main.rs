use clap::{arg, Command};

fn main() {
    let matches = Command::new("rs-echo")
        .version("0.0.1")
        .about("Echo implementation in Rust")
        .arg(arg!(<TEXT>...).help("Input text").required(true))
        .arg(arg!(--omit_newline).short('n').help("Do not print newline"))
        .get_matches();

    let text: Vec<String> = matches.get_many::<String>("TEXT").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    println!("{}{}", text.join(" "), ending);
    //println!("{:#?}", matches);
}
