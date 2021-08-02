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
