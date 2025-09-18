### Notes
seq 1 20

https://github.com/tokio-rs/tokio


lines per second (--rate N)

interval seconds (--interval X)



seq 1 20 | cargo run -- --rate 1





while true; do
    date +"%Y-%m-%d %H:%M:%S.%3N" >> test.log
    sleep 0.1
done
