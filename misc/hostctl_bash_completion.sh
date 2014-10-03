#####################################################################
###
### Bash Autocompletion for hostctl
### include this file in your .bash_profile or .bashrc to use completion
###
_hostctl()
{
    local cur=${COMP_WORDS[COMP_CWORD]}
    local prev=${COMP_WORDS[COMP_CWORD-1]}
    local next=""

    local all_options="--command -c --executelocal -e --recipe -r --nodes -n --debug -d --quiet -q --help -h --show -s --array -a --login -l --optssh -o --batchmode -b"   
    all_options="$all_options --prompt -p --makeselection -m --inscreen -i --term -t --wait -w"
    local all_groups="$(hostctl -s 2>/dev/null|grep "Group"|awk '{print $2}'|xargs)"

     case $prev in
      --command|-c|--executelocal|-e|--optssh|-o|--wait|-w|--inscreen|-i)
         next=""
      ;;
      --recipe|-r)
         next="$(hostctl 2>/dev/null| egrep "^ .*[ ][ ]*/"|awk '{print $1}'|xargs)"
      ;;
      --nodes|-n)
         next="$(hostctl -s --array all 2>/dev/null)"
      ;;

      *)
         next="$all_options $all_groups"
         ;;
     esac  

    # Sort the options
    COMPREPLY=( $(compgen -W "$next" -- $cur) )
}
complete -F _hostctl hostctl 
