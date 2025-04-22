use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "Generate code from DSL",
    rename_all = "kebab-case"
)]
struct Args {
    input: String,
    output: String,
    #[arg(short, long, default_value_t = true)]
    c: bool,
    #[arg(short = 'p', long, default_value_t = true)]
    cpp: bool,
    #[arg(short, long, default_value_t = true)]
    json_schema: bool,
}

fn main() {
    let args = Args::parse();
    lib::main(
        &args.input,
        &args.output,
        args.c,
        args.cpp,
        args.json_schema,
    );
}
