#!/bin/bash

for i in {1..100}
do
    echo -e '{"time":"2019-07-14T19:45:37+0100", "level":"warning","thread": 4, "message":"Warning message here", "tags":["tag1", "tag2"]}' >> out.json_lines
    sleep 1;
done