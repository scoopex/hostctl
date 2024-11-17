use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version,
    about = "convenient management and execution of command on groups of hosts",
    after_help = r###"
    EXAMPLES

    Show disk usage on all systems which belong to group 'foobar'
        "hostctl -c 'df -h' foobar"

    Show disk usage on all systems which belong to group 'foobar', ask after
    every node what to do next
        "hostctl -p -c 'df -h' foobar"

    Execute ncurses application 'top' on all systems which belong to group
    'foobar' using a pseudo tty
        "hostctl -t -c 'top' foobar"

    Copy files using rsync to a remotehost by replacing the string HOST with the
    current hostname of the systems
        "hostctl -e -c 'rsync -avP /tmp/foo HOST:/tmp/bar' foobar"

    Execute script/recipe 'apache_status' on all systems which belong to
    group 'foobar' (shortcut or explictit path)
        "hostctl -r apache_status foobar"
        "hostctl -r /foo/bar/baz/apache_status foobar"

    Login sequentially on all hosts which belong to group 'foobar'
        "hostctl -l foobar"

    Start a screen session with 'top' on all systems which belong to group
    'foobar'
        "hostctl -c 'top' --inscreen my-magic-screen foobar"
        "screen -x my-magic-screen"

    ENVIRONMENT VARIABLES

    HOSTCTL_CONFIG
        Define a alternate configuration location directory

        Default search order: ~/.hostctl/hostctl.conf, <HOSTCTL BINARY DIRECTORY>/hostctl.conf

    "###
)]
pub struct CommandLineArgs {
    /// Command to execute. A ssh login is performed on the specified hosts and the specified
    /// command is executed on the remotehost.
    #[arg(short, long, default_value = "")]
    pub(crate) command: String,

    /// Execute as local command and add hostname to the command
    /// (Hostname will be appended to the command, or inserted where 'HOST' string
    /// is located in the string.
    #[arg(short, long)]
    pub(crate) execute_local: bool,

    /// Recipe to execute. A recipe is a shellscript which defines recurring administration tasks.
    /// Recipes can be stored in $HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and can
    /// be called by their basename.
    /// Alternatively, recipes can be called be their fully qualified path.
    #[arg(short, long, default_value = "")]
    pub(crate) recipe: String,

    /// Specify hosts instead of groups.
    #[arg(short, long, )]
    pub(crate) nodes: bool,

    /// show group(s)
    #[arg(short, long)]
    pub(crate) show: bool,

    /// Output an array list od nodes
    #[arg(short, long)]
    pub(crate) array: bool,

    /// output hosts in json format
    #[arg(short, long)]
    pub(crate) json: bool,

    /// debug mode
    #[arg(short, long)]
    pub(crate) debug: bool,

    /// disable automatic detection of interactive usage and output colors always
    #[arg(short, long)]
    pub(crate) forcecolor: bool,

    /// reduce output, useful for aggregating output of multiple hosts
    #[arg(short, long)]
    pub(crate) quiet: bool,

    // batchmode, no password prompting (skip host if not ssh-key auth is possible)
    #[arg(short, long)]
    pub(crate) batchmode: bool,

    /// start command/script screen session
    // (screen -x <session>, see 'man screen' or 'STRG+a :help')
    #[arg(short, long, default_value = "")]
    pub(crate) inscreen: String,

    /// Add the arguments to the ssh command.
    #[arg(short, long, default_value = "")]
    pub(crate) optssh: String,

    // login to each host
    // (sleep 1 second after every login, use STRG+c to terminate iteration)
    #[arg(short, long)]
    pub(crate) login: bool,

    // use sudo to gather root privileges
    #[arg(long)]
    pub(crate) sudo: bool,

    // Force pseudo-tty allocation
    // (typically needed for tools which use (ncurses-)text-menus)
    #[arg(short, long)]
    pub(crate) term: bool,

    /// wait a specified number of seconds before continuing at next host
    #[arg(short, long, default_value = "0")]
    pub(crate) wait: u64,

    /// ask after every execution, if hostctl should continue, retry, execute a shell or quit
    #[arg(short, long)]
    pub(crate) prompt: bool,

    /// raise a prompt before each host which provides the possibility to
    /// confirm or skip the host or quit execution
    #[arg(short, long)]
    pub(crate) makeselection: bool,

    /// The log level
    #[arg(long, default_value = "info")]
    pub(crate) log_level: String,

    /// Groups or nodes for the iteration
    #[arg()]
    pub(crate) items: Vec<String>,
}
