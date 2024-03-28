use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version,

about = "convenient management and execution of command on groups of hosts",

long_about = r#"hostctl is a generic tool which supports batched execution of tasks
    on groups of hosts.

    Examples:
        Show disk usage on all systems which belong to group 'foobar'
            "hostctl -c 'df -h' foobar"

        Show disk usage on specific nodes
            "hostctl -c 'df -h' -n node01 node03 node05"

        Show a aggregated view of all cronjobs running on a cluster
            "hostctl -c "crontab -l -u www-user" webcluster01 -q | less"

        Show disk usage on all systems which belong to group 'foobar', ask after
        every node what to do next
            "hostctl -p -c 'df -h' foobar"

        Execute ncurses application 'top' on all systems which belong to group
        'foobar'
            "hostctl -t -c 'top' foobar"

        Copy files using rsync to a remotehost
            "hostctl -e 'rsync -avP /tmp/foo HOST:/tmp/bar' foobar"

        Execute script/recipe 'apache_status' on all systems which belong to
        group 'foobar'
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
        Define a alternate configuration location

        Default search order:

         1. ~/.hostctl/hostctl.conf
         2. <HOSTCTL_BINARY_DIRECTORY>/hostctl.conf

    HOSTCTL_CONFIG_DYNAMIC_SCRIPT
        Get additional group definitions by reading stdout of the given
        script

"#,
)]
pub struct CommandLineArgs {
    /// Command to execute. A ssh login is performed on the specified hosts and the specified
    /// command is executed on the remotehost.
    #[arg(short, long, required = false)]
    pub(crate) command: String,

    // Execute a local command and add hostname to the command
    /// (Hostname will be appended to the command, or inserted where 'HOST' string
    /// is located in the string.
    #[arg(short, long, default_value = "")]
    pub(crate) executelocal: String,

    /// Recipe to execute. A recipe is a shellscript which defines recurring administration tasks.
    /// Recipes can be stored in $HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and can
    /// be called by their basename.
    /// Alternatively, recipes can be called be their fully qualified path.
    #[arg(short, long, default_value = "")]
    pub(crate) recipe: String,

    /// Specify hosts instead of groups.
    #[arg(short, long)]
    pub(crate) nodes: Vec<String>,

    /// show group(s)
    #[arg(short, long, default_value = "all")]
    pub(crate) show: Vec<String>,

    /// debug mode
    #[arg(short, long)]
    pub(crate) debug: bool,

    /// disable automatic detection of interactive usage and output colors always
    #[arg(short, long)]
    pub(crate) forcecolor: bool,


    /// reduce output, useful for aggregating output of multiple hosts
    #[arg(short, long)]
    pub(crate) quiet: bool,

    ///
    #[arg(short, long)]
    pub(crate) array: bool,

    // batchmode, no password prompting (skip host if not ssh-key auth is possible)
    #[arg(short, long)]
    pub(crate) batchmode: bool,

    /// start command/script screen session
    // (screen -x <session>, see 'man screen' or 'STRG+a :help')
    #[arg(short, long, default_value = "")]
    pub(crate) inscreen: String,

    /// Add the arguments to the ssh command.
    #[arg(short, long, default_value= "")]
    pub(crate) optssh: String,

    // login to each host
    // (sleep 1 second after every login, use STRG+c to terminate iteration)
    #[arg(short, long)]
    pub(crate) login: bool,

    // Force pseudo-tty allocation
    // (typically needed for tools which use (ncurses-)text-menus)
    #[arg(short, long)]
    pub(crate) term: bool,

    // wait a specified number of seconds before continuing at next host
    #[arg(short, long)]
    pub(crate) wait: bool,

    // ask after every execution, if hostctl should (c)ontinue, (r)etry, (s)hell, (q)uit, (e)dit
    #[arg(short, long)]
    pub(crate) prompt: bool,

    // raise a prompt before each host which provides the possibility to
    // confirm, skip or quit execution
    #[arg(short, long)]
    pub(crate) makeselection: bool,

    /// The log level
    #[arg(long, default_value = "info")]
    pub(crate) log_level: String,

}
