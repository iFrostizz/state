use cfg::Bytecode;
use std::{
    env::{self, Args},
    iter::Skip,
};

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    bytecode: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    bytecode: String,
}

impl From<ConfigBuilder> for Config {
    fn from(value: ConfigBuilder) -> Self {
        Config {
            bytecode: value.bytecode.expect("bytecode missing !"),
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let mut builder = ConfigBuilder::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-b" | "--bytecode" => {
                let code = get_next_arg(&mut args, "-b");
                builder.bytecode = Some(code);
            }
            _ => panic!("unrecognized arg `{arg}`"),
        }
    }
    let config: Config = builder.into();
    let bytecode: Bytecode = config.bytecode.as_str().into();
    let mnemonics = bytecode.as_mnemonics();
    dbg!(&mnemonics);
}

fn get_next_arg<'a>(args: &'a mut Skip<Args>, flag: &'a str) -> String {
    args.next()
        .unwrap_or_else(|| panic!("missing arg after flag! {flag}"))
}
