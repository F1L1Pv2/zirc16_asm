use std::path::PathBuf;
use argp::FromArgs;

/// Assembler made for zirc* architectures developed by Kaktus14
#[derive(FromArgs, Debug)]
pub struct Args {
    /// Name of cpu architecture
    #[argp(positional)]
    pub isa: String,

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

        args.output = match args.output {
            Some(value) => Some(value.with_extension(format!("{}.bin", &args.isa))),
            None => Some(args.input.with_extension("").with_extension(format!("{}.bin", &args.isa)))
        };

        args
    }
}
