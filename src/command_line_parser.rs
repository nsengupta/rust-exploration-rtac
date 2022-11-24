use std::env;
use text_colorizer::Colorize;


pub struct Arguments {
    pub ip_file_name: String
}

fn replace(target:&str, replace_with: &str, text: &str) -> Result<String, regex::Error>{
    let regex_compiled = regex::Regex::new(target)?;
    Ok(regex_compiled.replace_all(text, replace_with).to_string())
}

pub(crate) fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        show_usage();
        eprintln!("{}: Insufficient parameters. Expected 1, got {}",
                  "Error".red().bold(),
                  args.len()
        );
        std::process::exit(-1);
    }

    Arguments {
        ip_file_name: args[0].clone()
    }
}

pub fn show_usage() -> () {
    eprintln!("{} - emit contents of a file from end, backwards",
              "rtac".green()
    );
    eprintln!("Usage: rtac <input file> ");
}
