#!/bin/bash
set -e

echo $SERVER
if [ "$1" = 'splitterrust_server' ]; then
    chown -R splitterrust: "$SERVER"
    su splitterrust -s /bin/bash -c "$SERVER"
fi

exec "$@"
