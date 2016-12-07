FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/right-back /web/
EXPOSE 80
ENTRYPOINT ["/web/right-back"]
