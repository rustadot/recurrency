#!/bin/bash

set -e

PID=$(lsof -i tcp:9944 | grep recurrency | xargs | awk '{print $2}')

if [ -n "${PID}" ]
then
    kill -9 ${PID}
    echo "Recurrency has been killed. 💀"
fi
