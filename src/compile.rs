use crate::Stmt;
use qbe::Module;

pub fn compile(ast: &[Stmt]) -> Module {
    let mut module = Module::new();
    for expr in ast {
        match expr {
            Stmt::Declare { name, value } => todo!(),
            Stmt::Call { name, args } => todo!(),
            Stmt::Function { name, args, body } => todo!(),
        }
    }
    module
}
