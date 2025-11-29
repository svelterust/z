use crate::{Atom, Statement, parse::Node};
use qbe::{DataDef, DataItem, Function, Instr, Linkage, Module, Type, Value};

pub fn compile(ast: &[Node]) -> Module<'_> {
    let mut module = Module::new();
    for expr in ast {
        match expr {
            Node::Function { name, body, .. } => {
                let mut func = Function::new(Linkage::public(), name, Vec::new(), Some(Type::Word));
                func.add_block("start");

                for statement in body {
                    match statement {
                        Statement::Declare { name, value } => {
                            let items = match value {
                                Atom::Symbol(_name) => todo!(),
                                Atom::String(string) => vec![
                                    (Type::Byte, DataItem::Str(string.into())),
                                    (Type::Byte, DataItem::Const(0)),
                                ],
                                Atom::Number(_number) => todo!(),
                                Atom::Float(_float) => todo!(),
                            };
                            let data = DataDef::new(Linkage::private(), name, None, items);
                            module.add_data(data);
                        }
                        Statement::Call { name, args } => {
                            let items = args
                                .into_iter()
                                .map(|arg| match arg {
                                    Atom::Symbol(name) => (Type::Long, Value::Global(name.into())),
                                    Atom::String(_string) => todo!(),
                                    Atom::Number(_number) => todo!(),
                                    Atom::Float(_float) => todo!(),
                                })
                                .collect::<Vec<_>>();

                            func.add_instr(Instr::Call(name.into(), items, None));
                        }
                    }
                }

                func.add_instr(Instr::Ret(Some(Value::Const(0))));
                module.add_function(func);
            }
        }
    }
    module
}
