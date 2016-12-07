# right-back
A simple standalone web server to be used when your website is during maintenance.

## Run code
```
pacman -S rustup
rustup install nightly
rustup default nightly
cargo run
```

## Build and run docker image
```
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:nightly'
rust-musl-builder cargo build --release
docker build -t right-back .
docker run -p 80:80 right-back
```
