#!/bin/bash

if [ "$1" = "web" ]; then
    cd web
    bun dev

else
    cd core
    cargo watch -x run

fi