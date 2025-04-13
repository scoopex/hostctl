complete -c hostctl -s c -l command -d 'Command to execute. A ssh login is performed on the specified hosts and the specified command is executed on the remotehost' -r
complete -c hostctl -s r -l recipe -d 'Recipe to execute. A recipe is a shellscript which defines recurring administration tasks. Recipes can be stored in $HOME/.hostctl/recipe/ or <hostctl-installation-path>/recipe/ and can be called by their basename. Alternatively, recipes can be called be their fully qualified path' -r -F
complete -c hostctl -s i -l inscreen -d 'start command/script screen session' -r
complete -c hostctl -s o -l optssh -d 'Add the arguments to the ssh command' -r
complete -c hostctl -s w -l wait -d 'wait a specified number of seconds before continuing at next host' -r
complete -c hostctl -l log-level -d 'The log level' -r
complete -c hostctl -s e -l execute-local -d 'Execute as local command and add hostname to the command (Hostname will be appended to the command, or inserted where \'HOST\' string is located in the string'
complete -c hostctl -s n -l nodes -d 'Specify hosts instead of groups'
complete -c hostctl -s s -l show -d 'show group(s)'
complete -c hostctl -l for-completion -d 'output for shell completion'
complete -c hostctl -s a -l array -d 'Output an array list od nodes'
complete -c hostctl -s j -l json -d 'output hosts in json format'
complete -c hostctl -s d -l debug -d 'debug mode'
complete -c hostctl -s f -l forcecolor -d 'disable automatic detection of interactive usage and output colors always'
complete -c hostctl -s q -l quiet -d 'reduce output, useful for aggregating output of multiple hosts'
complete -c hostctl -s b -l batchmode
complete -c hostctl -s l -l login -d 'login to each host (sleep 1 second after every login, use STRG+c to terminate iteration)'
complete -c hostctl -l sudo -d 'use sudo to gather root privileges'
complete -c hostctl -s t -l term -d 'Force pseudo-tty allocation (typically needed for tools which use (ncurses-)text-menus)'
complete -c hostctl -s p -l prompt -d 'ask after every execution, if hostctl should continue, retry, execute a shell or quit'
complete -c hostctl -s m -l makeselection -d 'raise a prompt before each host which provides the possibility to confirm or skip the host or quit execution'
complete -c hostctl -s k -l knownhostsaccept -d 'accept all ssh known hosts keys'
complete -c hostctl -s h -l help -d 'Print help'
complete -c hostctl -s V -l version -d 'Print version'
