mod classfile;
mod command;
mod runtime;

fn main() {
    let arg = command::parse();
    println!("{:#?}", arg);
}
