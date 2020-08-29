# fprompt

### unadded
![](images/2.png)

### uncommited
![](images/3.png)

### unpushed
![](images/4.png)

### unadded & unpushed
![](images/1.png)


## Install

1. append following code to your `.bashrc`

```sh
export PATH='$HOME/.cargo/bin:$PATH"
export PROMPT_COMMAND="fprompt --home $HOME --pwd $PWD --error $? --width $COLUMNS'
```
