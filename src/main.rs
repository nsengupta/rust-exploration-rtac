mod command_line_parser;

fn main() {

    println!("Hello, world!");

    let args = command_line_parser::parse_args();

    println!("input file's name {}", args.ip_file_name);
}
