use toukei::cli::Cli;


fn main() {
    let mut cli = Cli::new();

    match cli.run() {
        Ok(_) => println!("CLI executed successfully"),
        Err(e) => eprintln!("CLI error: {}", e),
    }
}