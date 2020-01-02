#!/bin/bash

docker build ./.devcontainer -t local-rust-env
docker run -it -v "$PWD:/app" -w "/app" local-rust-env cargo $@ 
