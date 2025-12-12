use toukei::cli::Cli;


fn main() {
    let mut cli = Cli::new();
    
    cli.run().unwrap();
}