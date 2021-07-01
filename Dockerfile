FROM alpine
COPY target/release/wikilinks_rust .
COPY data.bin .
CMD ["wikilinks_rust"]