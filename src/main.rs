use colored::*;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    message: String,
    #[structopt(short = "d", long = "dead")]
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,
}
fn cat(catfile: &Option<std::path::PathBuf>, eye: ColoredString) {
    match &catfile {
        Some (path) => {
            let cat_template =
                std::fs::read_to_string(path).expect(&format!("could not read file {:?}", path));

            let cat_picture = cat_template.replace("{eye}", &eye.to_string());
            println!("{}", &cat_picture);
        }
        None => {
            println!("      \\");
            println!("       \\");
            println!("          /\\_/\\");
            println!("         ( {eye} {eye} )");
            println!("         =( I )=");
        }
    }
}
fn main() {
    let options = Options::from_args(); // [2]
    let message = options.message;
    if message.to_lowercase() == "woof" {
        eprintln!("The cat cant say woof the fuck")
    }
    println!("{}", message.bright_yellow().underline().on_green());
    let eye = {if options.dead { "x" } else { "o" }}.green();
    let catfile = &options.catfile;
    cat(catfile, eye)
}