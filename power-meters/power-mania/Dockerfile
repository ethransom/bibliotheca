FROM golang:1.17-buster as builder

WORKDIR /app

COPY go.* ./
RUN go mod download

# prebuild for layer caching
RUN go build github.com/mattn/go-sqlite3

COPY **.go ./

RUN go build -v

FROM debian:buster-slim
RUN set -x && apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/power-mania /app/power-mania
COPY www /app/www/

EXPOSE 8080

WORKDIR /app

CMD ["/app/power-mania"]
