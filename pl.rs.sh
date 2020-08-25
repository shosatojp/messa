target=$(dirname $BASH_SOURCE)/target/debug/pl

function create_ps1() {
    PS1=$("$target" '--home' "$HOME" '--pwd' "$PWD" '--error' $? '--width' "$COLUMNS")
}