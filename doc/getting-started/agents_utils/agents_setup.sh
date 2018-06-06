#!/bin/bash
LOGS_DIR=/home/indy/logs

mkdir -p $LOGS_DIR

startagent () {
    local NAME=$1
    local PORT=$2
    echo Starting agent \"$NAME\" on port $PORT
    python3 "/usr/local/lib/python3.5/dist-packages/indy_client/test/agent/$NAME.py" --port $PORT > $LOG_DIR/$NAME.log
    echo Agent \"$NAME\" has quit with exit code $?
}

# Start Nodes
/usr/bin/supervisord &

# Configure the pool
indy-cli ~/getting_started.indyscript

# Set up agents
startagent faber 9702 &
startagent acme 9706 &
startagent thrift 9708 &

# Keep container alive
wait



