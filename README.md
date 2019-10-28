# JSON Log Viewer
## Description
Simple JSON Log viewer written in Rust. It prints out formatted and colored log to stdout.
## Usage
Example of input data:
```javascript
{"time":"2019-07-14T19:43:37+0100", "level":"error","thread": 1, "message":"Error message here", "tags":["tag1", "tag2", "tag3"], "context":{ "error": "Some error here"}}
{"time":"2019-07-14T19:45:37+0100", "level":"error", "thread": 1, "message":"Another error message", "tags":["tag1"]}
{"time":"2019-07-14T19:47:37+0100", "level":"info","thread": 1, "message":"Info message", "tags":["tag1", "tag3"]}
{"time":"2019-07-14T19:43:37+0100", "level":"debug","thread": 3, "message":"Debug message", "tags":["tag2", "tag3"]}
{"time":"2019-07-14T19:45:37+0100", "level":"warning","thread": 4, "message":"Warning message here", "tags":["tag1", "tag2"]}
{"time":"2019-07-14T19:45:37+0100", "level":"trace","thread": 4, "message":"Trace message", "tags":["tag1", "tag2", "tag3"]} 
```

Command:

```bash
jlv log.txt
```

Result:

```
TIME    LEVEL   THREAD  MESSAGE TAGS    CONTEXT
[2019-07-14T19:43:37+0100][E][1][Error message here][["tag1","tag2","tag3"]][{"error":"Some error here"}]
[2019-07-14T19:45:37+0100][E][1][Another error message][["tag1"]]
[2019-07-14T19:47:37+0100][I][1][Info message][["tag1","tag3"]]
[2019-07-14T19:43:37+0100][D][3][Debug message][["tag2","tag3"]]
[2019-07-14T19:45:37+0100][W][4][Warning message here][["tag1","tag2"]]
[2019-07-14T19:45:37+0100][T][4][Trace message][["tag1","tag2","tag3"]]
```

Flags:

* -f (-follow) - the program does not exit on EOF
