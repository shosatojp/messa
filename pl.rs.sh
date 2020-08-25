function create_ps1() {
    PS1=$($(dirname $BASH_SOURCE)/target/debug/pl --home $HOME --pwd $PWD --error $? --width `tput cols`)
}