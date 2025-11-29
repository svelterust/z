use z::Result;

enum Command {
    Check,
    Compile,
    Run,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" => Ok(Command::Check),
            "compile" => Ok(Command::Compile),
            "run" => Ok(Command::Run),
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

fn build(args: &Args) -> Result<String> {
    // Build the project
    let input = std::fs::read_to_string(&args.file)?;
    let ast = z::parse(&input)?;
    let module = z::compile(&ast);

    // Write to disk
    let name = args
        .file
        .file_stem()
        .and_then(|it| it.to_str())
        .ok_or_else(|| format!("Invalid file name: {}", args.file.display()))?;
    let path = format!("build/{name}.ssa");
    std::fs::write(&path, module.to_string())?;
    let output = format!("build/{name}");

    // Process with QBE, compile with TinyCC then run
    // qbe -o {name}.s {name}.ssa
    // tcc {name}.s -o {name}
    std::process::Command::new("qbe")
        .args(["-o", &format!("build/{name}.s"), &path])
        .status()?;
    std::process::Command::new("tcc")
        .args([&format!("build/{name}.s"), "-o", &output])
        .status()?;
    Ok(output.to_string())
}

fn main() -> Result<()> {
    // Get args
    let Ok(args) = Args::new() else {
        println!("Usage: z [command] [file]");
        std::process::exit(1);
    };

    // Compile
    match args.command {
        Command::Check => {
            // Parse and output
            let input = std::fs::read_to_string(args.file)?;
            let ast = z::parse(&input)?;
            println!("{ast:#?}\n");
            let module = z::compile(&ast);
            print!("{module}");
        }
        Command::Compile => {
            build(&args)?;
        }
        Command::Run => {
            let path = build(&args)?;
            std::process::Command::new(path).status()?;
        }
    }
    Ok(())
}
