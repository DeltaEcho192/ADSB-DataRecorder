FROM arm64v8/rust

WORKDIR /usr/src/adsb
COPY . .

RUN cargo install --path .

CMD ["adsb_database"]