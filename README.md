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

# Manpage:
(output of "hostctl -h")
```
License - see: LICENSE.txt


NAME
    hostctl - convenient management and execution of comand on groups of hosts

SYNOPSIS
    hostctl [OPTIONS] [GROUPNAMES]

    To specify more than one <groupname>, specify groups separated by spaces
    or comma. Options and groupnames can be mixed. Options can be
    abbreviated if there are no ambiguities.

OPTIONS
    --command <command>
        Command to execute. A ssh login is performed on the specified hosts
        and the specified command is executed on the remotehost.

    --executelocal <command>
        Execute a local command and add hostname to the command (Hostname
        will be appended to the command, or inserted where 'HOST' string is
        located in the string.

    --recipe <name>|<path>
        Recipe to execute. A recipe is a shellscript which defines recurring
        administration tasks. Recipes can be stored in
        $HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and
        can be called by their basename. Alternatively, recipes can be
        called be their fully qualified path.

    --nodes
        Specify hosts instead of groups.

    --debug
        debug mode

    --quiet
        reduce output, useful for aggregating output of multiple hosts

    --help
        show this manpage

    --show
        show group(s)

    --show --array
        list hosts suitable for array iteration

    --login
        login to each host (sleep 1 second after every login, use STRG+c to
        terminate iteration)

    --optssh <arguments>
        Add the arguments to the ssh command.

    --batchmode
        batchmode, no password prompting (skip host if not ssh-key auth is
        possible)

    --prompt
        ask after every execution, if hostctl should (c)ontinue, (r)etry,
        (s)hell, (q)uit

    --makeselection
        raise a prompt before each host which provides the possibility to
        confirm, skip or quit execution

    --inscreen <session>
        start command/script screen session (screen -x <session>, see 'man
        screen' or 'STRG+a :help')

    --term
        Force pseudo-tty allocation (typically needed for tools which use
        (ncurses-)text-menus)

    --wait <seconds>
        wait a specified number of seconds before continuing at next host

    --prompt
        raise a prompt after each host which provides the possibility to to
        open a shell, re-execute, quit

DESCRIPTION
    hostctl is a generic tool which supports batched execution of tasks on
    groups of hosts.

EXAMPLES
    Show disk usage on all systems which belong to group 'foobar'
        "hostctl -c 'df -h' foobar"

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

        Default search order: ~/.hostctl/hostctl.conf, <HOSTCTL BINARY
        DIRECTORY>/hostctl.conf

    HOSTCTL_CONFIG_DYNAMIC_SCRIPT
        Get additional group definitions by reading stdout of the given
        script

BUGS
    features :-)

AUTHOR
    Marc Schoechlin <ms@256bit.org>

    Marc Schoechlin <marc.schoechlin@dmc.de>

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


