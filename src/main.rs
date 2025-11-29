use qbe::{DataDef, DataItem, Function, Instr, Linkage, Module, Type, Value};
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

fn generate_main(module: &mut Module) {
    let mut func = Function::new(Linkage::public(), "main", Vec::new(), Some(Type::Word));

    func.add_block("start");
    func.add_instr(Instr::Call(
        "printf".into(),
        vec![(Type::Long, Value::Global("name".into()))],
        None,
    ));
    func.add_instr(Instr::Ret(Some(Value::Const(0))));

    module.add_function(func);
}

fn generate_name(module: &mut Module) {
    let items = vec![
        (Type::Byte, DataItem::Str("Odd-Harald".into())),
        (Type::Byte, DataItem::Const(0)),
    ];
    let data = DataDef::new(Linkage::private(), "name", None, items);
    module.add_data(data);
}

fn main() -> Result<()> {
    // Get args
    let Ok(args) = Args::new() else {
        println!("Usage: z [command] [file]");
        std::process::exit(1);
    };

    // Parse file
    let input = std::fs::read_to_string(args.file)?;
    let ast = z::parse(&input)?;
    println!("Got AST: {ast:#?}");

    // QBE
    let mut module = Module::new();
    generate_main(&mut module);
    generate_name(&mut module);
    println!("{module}");
    std::fs::write("output.ssa", module.to_string())?;
    Ok(())
}
