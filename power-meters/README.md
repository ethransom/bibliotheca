# `power-mania`

Files for my project to collect and visualize residential power meter consumption data.

# Raspberry Pi Setup

Install radio server:

```
sudo apt install rtl-sdr
```

Boot radio server:

```
sudo rtl_tcp -n 256 # crashes without setting size of linked list to 256 :/
```

Install rtlamr.

In another terminal, run uploader script:

```
./rtlamr -filterid=63021097 -format=json | xargs -d'\n' -I"{}" curl -H "Content-Type: application/json" -X POST -d'{}' 100.73.94.71:8080
```

How I cross-compiled this rtlamr binary for the pi:

```
OS=linux GOARCH=arm GOARM=6 go build
```
