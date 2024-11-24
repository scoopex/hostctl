hostctl - A ssh loop on steroids
===========

Convenient management and execution of command on groups of hosts : *A ssh loop on sterioids*
I created this tool more than one decade ago, because existing tools suck.
(Cluster Shell, Bolt, ... and probably because of NIH)

# Install and use

1. Download ar current release from the [releases page](https://github.com/scoopex/hostctl/releases)
2. Extract the archive 
   ```bash
   cd ${INSTALL_DIR?The installation base directory}
   tar xvf hostctl-v0.9.11.tar.gz
   ```
3. Add the tool to a directory which is in your PATH search-scope
   ```bash
   cd ${INSTALL_DIR?The installation base directory}/hostctl
   ln -snf $PWD/hostctl ~/bin/hostctl
   ```
4. Create directories
   ```
   mkdir -p ~/.hostctl/repices
   ```
5. Configure your environments:
   (See also "EXAMPLES" section)
    ```
    echo << 'EOF'
    #<groupname> : <host>, <host>, ...
    web2 :  barfoo-l01-ap01, barfoo-l01-ap02, barfoo-l01-ap03, barfoo-l01-ap04, barfoo-l01-ap05, barfoo-l01-ap06
    db2 :  barfoo-l01-db01, barfoo-l01-db02, barfoo-l01-db03
    EOF 
    ```
6. Add shell completion to your shell init
   (`~/.bashrc`, `~/.zshrc`)
   ```
   source ${INSTALL_DIR?The installation base directory}/hostctl_$( basename $SHELL)_completion.sh
   ```
6. Configure SSH
   * Use the "ssh-agent"
   * Activate ssh agent forwarding (openssh: `ForwardAgent yes`) on your desktop  
     ssh client and all systems you want to use "hostctl"
   * Activate ssh agent-forwarding (openssh: `AllowAgentForwarding yes`, this should be the default value)
     on your ssh servers 

Invoke hostctl to execute commands or scripts on the specified hosts.

# Develop

1. Install software prerequisites:
   ```
   sudo apt-get install rustc ssh screen 
   ```
2. Build it and add it to a directory which is in your PATH search-scope
   ```
   cd ${INSTALL_DIR?The installation base directory}/hostctl
   git clone https://github.com/scoopex/hostctl.git hostctl
   cd hostctl
   cargo build --release
   ln -snf ${INSTALL_DIR?The installation base directory}/hostctl/target/release/hostctl ~/bin/hostctl
   ```
3. Build completions
   ```
   target/release/hostctl generate-completions bash > misc/hostctl_bash_completion.sh
   target/release/hostctl generate-completions zsh > misc/hostctl_zsh_completion.sh
   target/release/hostctl generate-completions fish > misc/hostctl_fish_completion.sh
   echo "source $INSTALLDIR/hostctl/misc/hostctl_bash_completion.sh" >> .bashrc
   exec bash
   ```
4. Execute step 4 and later in the previous section   

# Help Output

(output of "hostctl --help")
```
$ hostctl --help

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
#<groupname> : <host>, <host>, ...
web2 :  barfoo-l01-ap01, barfoo-l01-ap02, barfoo-l01-ap03, barfoo-l01-ap04, barfoo-l01-ap05, barfoo-l01-ap06
db2 :  barfoo-l01-db01, barfoo-l01-db02, barfoo-l01-db03
```

 * Hostgroup web2 is only visible/usable on host jump-barfoo
 * Hostgroup db2 is only visible/usable on host jump-barfoo


# Missing features

- Implement completion of hosts and groups
- cluster shell mode with "--inscreen" : send STDIN of a master terminal to all screens
- packaging for rpm and deb
- Manual sorting of nodes
- Health check cmd for finishing the node

# Licence and Authors

Additional authors are very welcome - just submit your patches as pull requests.

 * Marc Schoechlin <ms@256bit.org>


