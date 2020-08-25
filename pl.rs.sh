target=$(dirname $BASH_SOURCE)/target/debug/pl

function create_ps1() {
    PS1=$("/home/sho/repos/powerline-shell/target/debug/pl" '--home' "$HOME" '--pwd' "$PWD" '--error' $? '--width' "$COLUMNS")
}