#!/bin/bash
set -e

if [ "$1" = 'server' ]; then
    chown -R splitterrust: "$SERVER"
    exec gosu splitterrust "$@"
fi

exec "$@"
