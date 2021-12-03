FROM rust:1.57.0

WORKDIR "/opt/aoc"

COPY . .

ENTRYPOINT ["cargo", "run", "--release"]