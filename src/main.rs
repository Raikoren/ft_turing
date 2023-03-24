use description::Description;
use std::{env, error::Error, fs};

mod description;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] == "help" {
        print!(
            "usage: ft_turing jsonfile input\n
positional arguments:
\tjsonfile\t\tjson description of the machine
\tinput\t\t\tinput of the machine\n
optional arguments:
\thelp\t\t\tshow this help message and exit"
        );
        return Ok(());
    }
    let src = &args[1];
    let contents = fs::read_to_string(src)?;
    let desc = Description::new(&contents)?;
    let input: Vec<char> = args[2].chars().collect();
    desc.check()?;
    desc.run(input);
    Ok(())
}
