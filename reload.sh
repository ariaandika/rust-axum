#!/bin/bash

# grep from ps, get the pid of the process, then signal sigusr1

ps -A | grep 'axum' | awk '{print $1}' | xargs kill -SIGUSR1
