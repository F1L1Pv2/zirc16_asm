use std::path::PathBuf;
use argp::FromArgs;

/// Assembler made for zirc* architectures developed by Kaktus14
#[derive(FromArgs, Debug)]
pub struct Args {
    /// Input file path
    #[argp(positional)]
    pub input: PathBuf,

    /// Output file path
    #[argp(positional)]
    pub output: Option<PathBuf>,
}

impl Args {
    pub fn parse() -> Self {
        let mut args: Args = argp::parse_args_or_exit(argp::DEFAULT);

        let isa: String = args.input.with_extension("").extension().unwrap().to_string_lossy().to_string();

        args.output = match args.output {
            Some(value) => Some(value.with_extension(format!("{}.bin", isa))),
            None => Some(args.input.with_extension("").with_extension(format!("{}.bin", isa)))
        };

        dbg!(isa);

        args
    }
}
