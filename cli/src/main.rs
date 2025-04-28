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
    #[arg(short = 'c', long, default_value_t = false)]
    skip_c: bool,
    #[arg(short = 'p', long, default_value_t = false)]
    skip_cpp: bool,
    #[arg(short = 'j', long, default_value_t = false)]
    skip_json_schema: bool,
}

fn main() {
    let args = Args::parse();
    lib::main(
        &args.input,
        &args.output,
        !args.skip_c,
        !args.skip_cpp,
        !args.skip_json_schema,
    );
}
