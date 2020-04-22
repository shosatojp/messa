#!/bin/bash

plsh_fgcolor(){
	echo "\e[38;$1m"
}

plsh_bgcolor(){
	echo "\e[48;$1m"
}

plsh_color(){
    case $1 in
        "red"        ) echo "2;244;67;54"  ;;
        "pink"       ) echo "2;233;30;99"  ;;
        "purple"     ) echo "2;156;39;176" ;;
        "deep_purple") echo "2;103;58;183" ;;
        "indigo"     ) echo "2;63;81;181"  ;;
        "blue"       ) echo "2;33;150;243" ;;
        "light_blue" ) echo "2;3;169;244"  ;;
        "cyan"       ) echo "2;0;188;212"  ;;
        "teal"       ) echo "2;0;150;136"  ;;
        "green"      ) echo "2;76;175;80"  ;;
        "light_green") echo "2;139;195;74" ;;
        "lime"       ) echo "2;205;220;57" ;;
        "yellow"     ) echo "2;255;235;59" ;;
        "amber"      ) echo "2;255;193;7"  ;;
        "orange"     ) echo "2;255;152;0"  ;;
        "deep_orange") echo "2;255;87;34"  ;;
        "brown"      ) echo "2;121;85;72"  ;;
        "grey"       ) echo "2;158;158;158";;
        "blue_grey"  ) echo "2;96;125;139" ;;
        
        "white"      ) echo "5;15";;
        "black"      ) echo "5;0";;
    esac
}

plsh_resetcolor(){
	echo "\e[0m"
}

plsh_default_bgcolor(){
	echo "\e[49m"
}

plsh_bold(){
	echo "\e[1m"
}

plsh_boldoff(){
	echo "\e[21m"
}

plsh_basic_git_branch_name(){
	name=`git rev-parse --abbrev-ref HEAD 2>/dev/null`
	if [ $? -eq 0 ];then
		echo "$name"
	fi
}

plsh_git_branch_name(){
    name=`plsh_basic_git_branch_name`
    if [ $name ];then
        GIT="$(plsh_bgcolor `plsh_color deep_orange`) \
$(plsh_fgcolor `plsh_color white`)\$(plsh_basic_git_branch_name)\
$(plsh_fgcolor `plsh_color deep_orange`)"
        echo $GIT
    fi
}

plsh_create_ps1(){
    export PS1_SRC="\
$(plsh_bgcolor `plsh_color indigo`)\
$(plsh_fgcolor `plsh_color white`) \u@\h \
$(plsh_fgcolor `plsh_color indigo`)\
\
$(plsh_bgcolor `plsh_color teal`)\
$(plsh_fgcolor `plsh_color white`) \w \
$(plsh_fgcolor `plsh_color teal`)\
\
$(plsh_bold)'\
\
$(plsh_git_branch_name)\
'$(plsh_boldoff)\
\
$(plsh_default_bgcolor)\
\
$(plsh_resetcolor)\n\
$(plsh_fgcolor `plsh_color white`)$(plsh_bgcolor `plsh_color red`)🤗 \$  \
$(plsh_resetcolor)$(plsh_fgcolor `plsh_color red`)\
$(plsh_resetcolor)\
 "
    PS1=$(eval "echo \"$PS1_SRC\"")
}
