#!/bin/bash

cd /opt/jencli

job_name=$(./jencli list | jq -r .[].name | fzf)

./jencli build --name $job_name