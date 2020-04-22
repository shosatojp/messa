#!/bin/bash

plsh_symbol_right='\ue0b0'
plsh_symbol_right_alt='\ue0b1'
plsh_symbol_branch='\ue0a0'

plsh_color_bg_userhost='indigo'
plsh_color_fg_userhost='white'
plsh_color_bg_path='teal'
plsh_color_fg_path='white'
plsh_color_bg_git='deep_orange'
plsh_color_fg_git='white'
plsh_color_bg_prompt='red'
plsh_color_fg_prompt='white'

plsh_fgcolor(){
	echo -n "\[\e[38;$1m\]"
}

plsh_bgcolor(){
	echo -n "\[\e[48;$1m\]"
}

plsh_color(){
    case $1 in
        "red"        ) echo -n "2;244;67;54"  ;;
        "pink"       ) echo -n "2;233;30;99"  ;;
        "purple"     ) echo -n "2;156;39;176" ;;
        "deep_purple") echo -n "2;103;58;183" ;;
        "indigo"     ) echo -n "2;63;81;181"  ;;
        "blue"       ) echo -n "2;33;150;243" ;;
        "light_blue" ) echo -n "2;3;169;244"  ;;
        "cyan"       ) echo -n "2;0;188;212"  ;;
        "teal"       ) echo -n "2;0;150;136"  ;;
        "green"      ) echo -n "2;76;175;80"  ;;
        "light_green") echo -n "2;139;195;74" ;;
        "lime"       ) echo -n "2;205;220;57" ;;
        "yellow"     ) echo -n "2;255;235;59" ;;
        "amber"      ) echo -n "2;255;193;7"  ;;
        "orange"     ) echo -n "2;255;152;0"  ;;
        "deep_orange") echo -n "2;255;87;34"  ;;
        "brown"      ) echo -n "2;121;85;72"  ;;
        "grey"       ) echo -n "2;158;158;158";;
        "blue_grey"  ) echo -n "2;96;125;139" ;;
        "white"      ) echo -n "5;15";;
        "black"      ) echo -n "5;0";;
    esac
}

plsh_resetcolor(){
	echo -n "\[\e[0m\]"
}

plsh_default_bgcolor(){
	echo -n "\[\e[49;24m\]"
}

plsh_bold(){
	echo -n "\[\e[1m\]"
}

plsh_boldoff(){
	echo -n "\[\e[21;24m\]"
}

plsh_basic_git_branch_name(){
	plsh_var_name=`git rev-parse --abbrev-ref HEAD 2>/dev/null`
	if [ $? -eq 0 ];then
		echo -n "$plsh_var_name"
	fi
    unset plsh_var_name
}

plsh_git(){
    plsh_var_name=`plsh_basic_git_branch_name`
    if [ $plsh_var_name ];then
        plsh_var_git="$(plsh_bgcolor `plsh_color $plsh_color_bg_git`)$plsh_symbol_right\
$(plsh_fgcolor `plsh_color $plsh_color_fg_git`) $plsh_symbol_branch $(plsh_basic_git_branch_name)$(plsh_git_status) \
$(plsh_fgcolor `plsh_color $plsh_color_bg_git`)"
        echo -n $plsh_var_git
    fi
    unset plsh_var_name
    unset plsh_var_git
}

plsh_git_status(){
    not_added=`git status -s | grep -e "^.\S" | wc -l`
    not_commited=`git status -s | grep -s "^[^? ]" | wc -l`
    not_pushed=`git cherry | wc -l`

    if [ "$not_added" != '0' ];then
        echo -n '*'
    fi
    if [ "$not_commited" != '0' ];then
        echo -n '+'
    fi
    if [ "$not_pushed" != '0' ];then
        echo -n " ↑$not_pushed"
    fi
    # if [ "$not_added" == '0' ] && [ "$not_commited" == '0' ] && [ "$not_pushed" == '0' ];then
    #     echo -n ''
    # fi
}

plsh_create_ps1(){


    plsh_var_dir=`pwd | sed "s|$HOME|~|"`
    plsh_var_dir=${plsh_var_dir//\// $plsh_symbol_right_alt }
    export plsh_var_ps1_src='\
$(plsh_bgcolor `plsh_color $plsh_color_bg_userhost`)\
$(plsh_fgcolor `plsh_color $plsh_color_fg_userhost`) \u@\h \
$(plsh_fgcolor `plsh_color $plsh_color_bg_userhost`)\
\
$(plsh_bgcolor `plsh_color $plsh_color_bg_path`)$plsh_symbol_right\
$(plsh_fgcolor `plsh_color $plsh_color_fg_path`) $plsh_var_dir \
$(plsh_fgcolor `plsh_color $plsh_color_bg_path`)\
\
$(plsh_git)\
$(plsh_default_bgcolor)$plsh_symbol_right\
\
$(plsh_resetcolor)\n\
$(plsh_fgcolor `plsh_color $plsh_color_fg_prompt`)$(plsh_bgcolor `plsh_color $plsh_color_bg_prompt`)🤗 \$ \
$(plsh_resetcolor)$(plsh_fgcolor `plsh_color $plsh_color_bg_prompt`)$plsh_symbol_right\
$(plsh_resetcolor) '
    PS1=$(eval "echo -en \"$plsh_var_ps1_src\"")
    unset plsh_var_dir
    unset plsh_var_ps1_src
}
