use clap::Parser;
use std::usize;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    input_file: PathBuf,

    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    min: usize,

    #[arg(long)]
    max: usize,

    #[arg(long)]
    alphabet: String,

    #[arg(long)]
    patterns: Vec<String>,

    #[arg(long)]
    t: Vec<usize>
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Archivo de salida de hashcat: {}", args.output.display());
        
    println!("Procesando archivo");

    for pattern in &args.patterns {
        for hash_type in args.t.iter() {
            let expanded_patterns = expand_variable_patterns(pattern, args.min, args.max);

            for pat in expanded_patterns {
                println!("Intentando: {} con : {}", pat, hash_type);
                match run_hashcat(&args.input_file, &args.output, &args.alphabet, &pat, hash_type) {
                    Some(result) => println!("[{}]: {}", pat, result),
                    None => println!("[{}]: Error desconocido", pat),
                }
            }
        }
    }


    println!("\nContenido del archivo de salida:");
    display_results(&args.output)?;

    Ok(())
}


fn expand_variable_patterns(base_pattern: &str, min: usize, max: usize) -> Vec<String> {
    let mut patterns = Vec::new();
    if base_pattern.contains("?x") {
        for len in min..=max {
            let replacement = "?1".repeat(len);
            let pat = base_pattern.replace("?x", &replacement);
            patterns.push(pat);
        }
    } else {
        patterns.push(base_pattern.to_string());
    }
    patterns
}

fn run_hashcat(
    file_path: &Path,
    output: &Path,
    alphabet: &str,
    pattern: &str,
    hash_type: &usize,
) -> Option<String> {
    let hashcat_cmd = get_hashcat_command();

    let output_result = Command::new(hashcat_cmd)
        .arg("-a")
        .arg("3")
        .arg("-m")
        .arg(hash_type.to_string())
        .arg("-w")
        .arg("3")
        .arg(file_path.as_os_str())
        .arg(pattern)
        .arg("--custom-charset1")
        .arg(alphabet)
        .arg("--force")
        .arg("-o")
        .arg(output.to_string_lossy().to_string())
        .output()
        .ok()?;

    if output_result.status.success() {
        Some("OK".to_string())
    } else {
        Some(format!(
            "Error: {}",
            String::from_utf8_lossy(&output_result.stderr)
        ))
    }
}

fn get_hashcat_command() -> String {
    if cfg!(target_os = "windows") {
        "hashcat.exe".to_string()
    } else {
        "hashcat".to_string()
    }
}

fn display_results(output: &Path) -> io::Result<()> {
    let file = File::open(output)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
