use env_logger::Env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The log level
    #[arg(long, default_value = "info")]
    log_level: String,

    // Output as array
    #[arg(short, long)]
    array: bool,

    // Output as array
    #[arg(short, long)]
    batchmode: bool,

    /// The log level
    #[arg(short, long, required = false)]
    command: String,

    // Output as array
    #[arg(short, long)]
    debug: bool,

    /// The log level
    #[arg(short, long, required = false)]
    executelocal: String,

    /// The log level
    #[arg(short, long, required = false)]
    inscreen: String,

    /// The log level
    #[arg(short, long, default_value= "")]
    optssh: String,

    /// The log level
    #[arg(short, long, required = false)]
    recipe: String,

    // Output as array
    #[arg(short, long)]
    login: bool,

    // Output as array
    #[arg(short, long)]
    quiet: bool,

    // Output as array
    #[arg(short, long)]
    term: bool,

    // Output as array
    #[arg(short, long)]
    forcecolor: bool,

    // Output as array
    #[arg(short, long)]
    wait: bool,

    // Output as array
    #[arg(short, long)]
    prompt: bool,

    // Output as array
    #[arg(short, long)]
    makeselection: bool,

    /// The log level
    #[arg(short, long)]
    nodes: Vec<String>,

}


fn main() {
    unsafe { libc::umask(0o077) };
    let args = Args::parse();

    env_logger::Builder::from_env(
        Env::default().default_filter_or(args.log_level)
    ).format_timestamp_secs().init();
}