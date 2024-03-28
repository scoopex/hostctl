use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// The log level
    #[arg(long, default_value = "info")]
    pub(crate) log_level: String,

    // Output as array
    #[arg(short, long)]
    pub(crate) array: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) batchmode: bool,

    /// The log level
    #[arg(short, long, required = false)]
    pub(crate) command: String,

    // Output as array
    #[arg(short, long)]
    pub(crate) debug: bool,

    /// The log level
    #[arg(short, long, default_value = "")]
    pub(crate) executelocal: String,

    /// The log level
    #[arg(short, long, default_value = "")]
    pub(crate) inscreen: String,

    /// The log level
    #[arg(short, long, default_value= "")]
    pub(crate) optssh: String,

    /// The log level
    #[arg(short, long, required = false, default_value = "")]
    pub(crate) recipe: String,

    // Output as array
    #[arg(short, long)]
    pub(crate) login: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) quiet: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) term: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) forcecolor: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) wait: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) prompt: bool,

    // Output as array
    #[arg(short, long)]
    pub(crate) makeselection: bool,

    /// The log level
    #[arg(short, long)]
    pub(crate) nodes: Vec<String>,
}
