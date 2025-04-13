#compdef hostctl

autoload -U is-at-least

_hostctl() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-c+[Command to execute. A ssh login is performed on the specified hosts and the specified command is executed on the remotehost]:COMMAND:_default' \
'--command=[Command to execute. A ssh login is performed on the specified hosts and the specified command is executed on the remotehost]:COMMAND:_default' \
'-r+[Recipe to execute. A recipe is a shellscript which defines recurring administration tasks. Recipes can be stored in \$HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and can be called by their basename. Alternatively, recipes can be called be their fully qualified path]:RECIPE:_files' \
'--recipe=[Recipe to execute. A recipe is a shellscript which defines recurring administration tasks. Recipes can be stored in \$HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and can be called by their basename. Alternatively, recipes can be called be their fully qualified path]:RECIPE:_files' \
'-i+[start command/script screen session]:INSCREEN:_default' \
'--inscreen=[start command/script screen session]:INSCREEN:_default' \
'-o+[Add the arguments to the ssh command]:OPTSSH:_default' \
'--optssh=[Add the arguments to the ssh command]:OPTSSH:_default' \
'-w+[wait a specified number of seconds before continuing at next host]:WAIT:_default' \
'--wait=[wait a specified number of seconds before continuing at next host]:WAIT:_default' \
'--log-level=[The log level]:LOG_LEVEL:_default' \
'-e[Execute as local command and add hostname to the command (Hostname will be appended to the command, or inserted where '\''HOST'\'' string is located in the string]' \
'--execute-local[Execute as local command and add hostname to the command (Hostname will be appended to the command, or inserted where '\''HOST'\'' string is located in the string]' \
'-n[Specify hosts instead of groups]' \
'--nodes[Specify hosts instead of groups]' \
'-s[show group(s)]' \
'--show[show group(s)]' \
'--for-completion[output for shell completion]' \
'-a[Output an array list od nodes]' \
'--array[Output an array list od nodes]' \
'-j[output hosts in json format]' \
'--json[output hosts in json format]' \
'-d[debug mode]' \
'--debug[debug mode]' \
'-f[disable automatic detection of interactive usage and output colors always]' \
'--forcecolor[disable automatic detection of interactive usage and output colors always]' \
'-q[reduce output, useful for aggregating output of multiple hosts]' \
'--quiet[reduce output, useful for aggregating output of multiple hosts]' \
'-b[]' \
'--batchmode[]' \
'-l[login to each host (sleep 1 second after every login, use STRG+c to terminate iteration)]' \
'--login[login to each host (sleep 1 second after every login, use STRG+c to terminate iteration)]' \
'--sudo[use sudo to gather root privileges]' \
'-t[Force pseudo-tty allocation (typically needed for tools which use (ncurses-)text-menus)]' \
'--term[Force pseudo-tty allocation (typically needed for tools which use (ncurses-)text-menus)]' \
'-p[ask after every execution, if hostctl should continue, retry, execute a shell or quit]' \
'--prompt[ask after every execution, if hostctl should continue, retry, execute a shell or quit]' \
'-m[raise a prompt before each host which provides the possibility to confirm or skip the host or quit execution]' \
'--makeselection[raise a prompt before each host which provides the possibility to confirm or skip the host or quit execution]' \
'-k[accept all ssh known hosts keys]' \
'--knownhostsaccept[accept all ssh known hosts keys]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
'*::items -- Groups or nodes for the iteration:_default' \
&& ret=0
}

(( $+functions[_hostctl_commands] )) ||
_hostctl_commands() {
    local commands; commands=()
    _describe -t commands 'hostctl commands' commands "$@"
}

if [ "$funcstack[1]" = "_hostctl" ]; then
    _hostctl "$@"
else
    compdef _hostctl hostctl
fi
