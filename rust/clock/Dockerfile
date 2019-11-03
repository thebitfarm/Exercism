FROM rust:1.38

WORKDIR /usr/src/clock

COPY . .
RUN cargo install --path .

CMD ["clock"]
 

