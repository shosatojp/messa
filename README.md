# PowerLine-Shell for Bash

create powerline prompt with **no dependency**.

![](images/5.png)

### unadded
![](images/2.png)

### uncommited
![](images/3.png)

### unpushed
![](images/4.png)

### unadded & unpushed
![](images/1.png)


## Install

1. place `pl.sh` as you like
2. append following code to your `.bashrc`

```sh
source $HOME/pl.sh # path to your `pl.sh` or link
export PROMPT_COMMAND=plsh_create_ps1
```
