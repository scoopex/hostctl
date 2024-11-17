hostctl - A ssh loop on steroids
===========

Convenient management and execution of command on groups of hosts : *A ssh loop on sterioids*
I created this tool more than one decade ago, because existing tools suck.
(Cluster Shell, Bolt, ... and probably because of NIH)

# Install

Software prerequisites:
 * rustc
 * ssh
 * screen

Install script:
 
On Ubuntu just do:
```
INSTALLDIR="/opt/"
sudo apt-get install rustc ssh screen 
cd ${INSTALLDIR?Installation Dir}
git clone https://github.com/scoopex/hostctl.git hostctl
cd hostctl
cargo build --release
ln -snf $INSTALLDIR/hostctl/target/release/hostctl /usr/local/bin/hostctl
target/release/hostctl generate-completions bash > misc/hostctl_bash_completion.sh
target/release/hostctl generate-completions zsh > misc/hostctl_zsh_completion.sh
target/release/hostctl generate-completions fish > misc/hostctl_fish_completion.sh
echo "source $INSTALLDIR/hostctl/misc/hostctl_bash_completion.sh" >> .bashrc
exec bash
```

Configure your environments:

 * Use the "ssh-agent"
 * Activate ssh agent forwarding (openssh: `ForwardAgent yes`) on your desktop  
   ssh client and all systems you want to use "hostctl"
 * Activate ssh agent-forwarding (openssh: `AllowAgentForwarding yes`, this should be the default value)
   on your ssh servers 

# Usage

Invoke hostctl to execute commands or scripts on the specified hosts.

See also "EXAMPLES" section.

# Help
(output of "hostctl --help")
```
$ hostctl --help

Run a binary or example of the local package

Usage: cargo run [OPTIONS] [ARGS]...

Arguments:
  [ARGS]...  Arguments for the binary or example to run

Options:
      --message-format <FMT>  Error format
  -v, --verbose...            Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                 Do not print cargo log messages
      --color <WHEN>          Coloring: auto, always, never
      --config <KEY=VALUE>    Override a configuration value
  -Z <FLAG>                   Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                  Print help

Package Selection:
  -p, --package [<SPEC>]  Package with the target to run

Target Selection:
      --bin [<NAME>]      Name of the bin target to run
      --example [<NAME>]  Name of the example target to run

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
      --keep-going              Do not abort the build as soon as there is an error
  -r, --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
      --target [<TRIPLE>]       Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --unit-graph              Output build graph in JSON (unstable)
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --lockfile-path <PATH>  Path to Cargo.lock (unstable)
      --ignore-rust-version   Ignore `rust-version` specification in packages
      --locked                Assert that `Cargo.lock` will remain unchanged
      --offline               Run without accessing the network
      --frozen                Equivalent to specifying both --locked and --offline

Run `cargo help run` for more detailed information.
marc@frink(2024-11-17 22:39:07)  ~/src/github/hostctl-rust [master]   
$ cargo run -- --help
   Compiling hostctl v0.1.0 (/home/marc/src/github/hostctl-rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.49s
     Running `target/debug/hostctl --help`
convenient management and execution of command on groups of hosts

Usage: hostctl [OPTIONS] [ITEMS]...

Arguments:
  [ITEMS]...  Groups or nodes for the iteration

Options:
  -c, --command <COMMAND>      Command to execute. A ssh login is performed on the specified hosts and the specified command is executed on the remotehost [default: ]
  -e, --execute-local          Execute as local command and add hostname to the command (Hostname will be appended to the command, or inserted where 'HOST' string is located in the string
  -r, --recipe <RECIPE>        Recipe to execute. A recipe is a shellscript which defines recurring administration tasks. Recipes can be stored in $HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and can be called by their basename. Alternatively, recipes can be called be their fully qualified path [default: ]
  -n, --nodes                  Specify hosts instead of groups
  -s, --show                   show group(s)
  -a, --array                  Output an array list od nodes
  -j, --json                   output hosts in json format
  -d, --debug                  debug mode
  -f, --forcecolor             disable automatic detection of interactive usage and output colors always
  -q, --quiet                  reduce output, useful for aggregating output of multiple hosts
  -b, --batchmode              
  -i, --inscreen <INSCREEN>    start command/script screen session [default: ]
  -o, --optssh <OPTSSH>        Add the arguments to the ssh command [default: ]
  -l, --login                  
      --sudo                   
  -t, --term                   
  -w, --wait <WAIT>            wait a specified number of seconds before continuing at next host [default: 0]
  -p, --prompt                 ask after every execution, if hostctl should continue, retry, execute a shell or quit
  -m, --makeselection          raise a prompt before each host which provides the possibility to confirm or skip the host or quit execution
      --log-level <LOG_LEVEL>  The log level [default: info]
  -h, --help                   Print help
  -V, --version                Print version


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

```

# Configuration file format (hostctl.conf):
```
#<perl-regex for visibility> : <groupname> : <host>, <host>, ...
foobar-l01-(ap|db)\d+ : web1 :  foobar-l01-ap01, foobar-l01-ap02, foobar-l01-ap03, foobar-l01-ap04, foobar-l01-ap05, foobar-l01-ap06
foobar-l01-(ap|db)\d+ : db1 :  foobar-l01-db01, foobar-l01-db02, foobar-l01-db03

jump-barfoo : web2 :  barfoo-l01-ap01, barfoo-l01-ap02, barfoo-l01-ap03, barfoo-l01-ap04, barfoo-l01-ap05, barfoo-l01-ap06
jump-barfoo : db2 :  barfoo-l01-db01, barfoo-l01-db02, barfoo-l01-db03
```

 * Hostgroup web1 is only visible/usable on hosts which match to regex "foobar-l01-(ap|db)\d+" - i.e. foobar-l01-ap99
 * Hostgroup db1 is only visible/usable on hosts which match to regex "foobar-l01-(ap|db)\d+" - i.e. foobar-l01-ap99
 * Hostgroup web2 is only visible/usable on host jump-barfoo
 * Hostgroup db2 is only visible/usable on host jump-barfoo


# Missing features

- cluster shell mode with "--inscreen" : send STDIN of a master terminal to all screens
- packaging for rpm and deb
- Show the next node on prompting
- Manual sorting of nodes
- Health check cmd for finishing the node


# Licence and Authors

Additional authors are very welcome - just submit your patches as pull requests.

 * Marc Schoechlin <ms@256bit.org>


