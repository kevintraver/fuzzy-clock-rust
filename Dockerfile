FROM alpine:3.6
MAINTAINER Kevin Traver "kevintraver@gmail.com"

ADD target/release/fuzzy_clock_rust .

CMD ["fuzzy_clock_rust"]
