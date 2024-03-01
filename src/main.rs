use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)?;
    // let content = match result {
    //     Ok(content) => { content }
    //     Err(error) => { return Err(error.into());  }
    // };

    // let content = std::fs::read_to_string(&args.path).unwrap();

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{:?}", line)
    //     }
    // }
    println!("file content {}", content);

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    Ok(())
}
