# Notes

## Todo
* Read about tokio
* Add a nice description including examples
* Improve the readme, embed a terminal demo
* Add some tests
* Add author, https://docs.rs/clap/latest/clap/_derive/index.html
* Allow using interval instead of rate

## Release process
* `cargo publish --dry-run --allow-dirty`
* `git tag -a 0.2.0 -m "Release version 0.2.0"`
* `cargo publish`

## Random
seq 1 20

https://github.com/tokio-rs/tokio


lines per second (--rate N)

interval seconds (--interval X)

seq 1 20 | cargo run -- --rate 1


while true; do
    date +"%Y-%m-%d %H:%M:%S.%3N" >> test.log
    sleep 0.1
done
