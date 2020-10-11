# fprompt

- Width flexible shell prompt

![](image.png)

## Install

1. Install `fprompt`

```sh
cargo install fprompt
```

2. Append following code to your `.bashrc`

```sh
export PATH="$HOME/.cargo/bin:$PATH"
function create_prompt(){
    PS1=$(fprompt --home "$HOME" --pwd "$PWD" --error "$?" --width "$COLUMNS" --user "$USER" --host "$HOSTNAME")
}
export PROMPT_COMMAND=create_prompt
```

3. Reload your shell

```sh
. ~/.bashrc
```

## Build

```sh
cargo build
```