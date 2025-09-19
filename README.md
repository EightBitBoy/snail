# snl / snail
`snl` ("snail") is like `tail -f` but slower 🐌!

Slow down the output of logs and other text streams in your terminal and make them easier to follow.

## Usage
`snl` is the right tool for you if you often view logs in real-time but the output is sometimes simply too fast to understand what is going on. With `snl` you can specify how many *lines per second* should appear on your termal.

Per default `snl` prints out *2 lines per second*, the rate is configurable.

`snl` reads from a single file or from `stdin`.

* Tail a log file: `snl /some/file.log`
* Use `snl` as a target for a pipe: `tail -f server.log | grep "error" | snl`
* Watch an application log, 5 lines per second: `snl /application.log -r 5`