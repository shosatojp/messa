# fprompt

[![CI](https://github.com/shosatojp/fprompt/actions/workflows/ci.yml/badge.svg)](https://github.com/shosatojp/fprompt/actions/workflows/ci.yml)

- Width flexible shell prompt
- Configure with yaml
- Supports ssh, git, kubernetes, datetime

![](image.png)

## Install

1. Install `fprompt`

- Download from [here](https://github.com/shosatojp/fprompt/releases/latest)
- or Use `cargo`

    ```sh
    cargo install fprompt
    ```

1. Append following code to your `.bashrc`

```sh
export PATH="$HOME/.cargo/bin:$PATH"
function create_prompt(){
    PS1=$(fprompt --error $? --width $COLUMNS --user $USER --host $HOSTNAME -c $HOME/.fprompt.yaml)
}
export PROMPT_COMMAND="create_prompt;$PROMPT_COMMAND"
```

2. Setup config

```sh
wget -O ~/.fprompt.yaml https://raw.githubusercontent.com/shosatojp/fprompt/master/.fprompt.yaml
```

3. Reload your shell

```sh
. ~/.bashrc
```

## Build

```sh
cargo build
```
