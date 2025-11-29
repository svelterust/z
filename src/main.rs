use z::Result;

enum Command {
    Compile,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compile" => Ok(Command::Compile),
            _ => Err(format!("Invalid command: {s}")),
        }
    }
}

struct Args {
    command: Command,
    file: std::path::PathBuf,
}

impl Args {
    fn new() -> Result<Args, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();
        let args = Args {
            command: pargs.free_from_str()?,
            file: pargs.free_from_str()?,
        };
        Ok(args)
    }
}

fn main() -> Result<()> {
    // Get args
    let Ok(args) = Args::new() else {
        println!("Usage: z [command] [file]");
        std::process::exit(1);
    };

    // Compile
    match args.command {
        Command::Compile => {
            let input = std::fs::read_to_string(args.file)?;
            let ast = z::parse(&input)?;
            for node in &ast {
                println!("{node}\n");
            }
            let module = z::compile(&ast);
            print!("{module}");
        }
    }
    Ok(())
}
