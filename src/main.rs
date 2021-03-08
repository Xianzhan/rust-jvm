mod classfile;
mod command;

fn main() {
    let arg = command::parse();
    println!("{:#?}", arg);
}
