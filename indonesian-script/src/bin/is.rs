use clap::Parser;
use indonesian_script::compile_is;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "is-cli")]
#[command(about = "Indonesian Script (.is) CLI Compiler", long_about = None)]
struct Cli {
    /// Input file (.is)
    input: PathBuf,

    /// Output file (.js)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let source = fs::read_to_string(&cli.input)
        .unwrap_or_else(|e| panic!("Gagal membaca file {:?}: {}", cli.input, e));

    let compiled = compile_is(&source);

    if let Some(out_path) = cli.output {
        fs::write(&out_path, compiled)
            .unwrap_or_else(|e| panic!("Gagal menulis file output {:?}: {}", out_path, e));
        println!("✅ Berhasil mengompilasi {:?} -> {:?}", cli.input, out_path);
    } else {
        println!("{}", compiled);
    }
}
