#!/bin/bash

job_name=$(./jencli list | jq -r .[].name | fzf)

./jencli build --name $job_name