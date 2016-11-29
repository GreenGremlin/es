#!/usr/bin/env bash
# Usage: esv rs [django]
# Summary:
# Help: Available options
#
# django:
#

set -e

# provide sub completions
if [ "$1" == "--complete" ]; then
    if [ -n "$(type -t workon)" ] && [ "$(type -t workon)" = "function" ]; then
        $(workon)
    fi
    exit
fi

if [ $# -lt 1  ]; then
    echo "Must specify a site to runserver"
    exit
fi

if [[ "$(hostname)" != "dev" ]]; then
    cd $ESV_ROOT
    vagrant ssh -- -t "/bin/bash && sudo su - django && esv rs $@"
else
    if [[ "$1" == *_dev ]]; then
        site="$1"
    else
        site="${1}_dev"
    fi
    if [ -n "$2" ]; then
        port="$1"
    else
        port="8081"
    fi

    # if [ "$(whoami)" != 'django' ]; then
    #     echo "Switching to django user"
    #     echo "sudo su - django -c \"source ~/.bashrc && esv rs ${@}\""
    #     sudo su - django -c "source ~/.bashrc && esv rs ${@}"
    #     # sudo su -l - django -c ". ~/.bashrc && echo \"\$TESTVAR\""
    #     exit
    # fi

    echo "$(type -t deactivate)"
    if [ -n "$(type -t deactivate)" ] && [ "$(type -t deactivate)" = "function" ]; then
        echo "Deactivating current venv"
        deactivate
    fi

    echo "Activating virtual environment for ${site}"

    . `which virtualenvwrapper.sh`
    workon $site && # without the && the script will exit here, not sure why

    echo "Starting server on port ${port}"
    python $PROJECT_ROOT/src/manage.py runserver 0.0.0.0:$port
fi
