```
./rtlamr -filterid=63021097 -format=json | xargs -d'\n' -I"{}" curl -H "Content-Type: application/json" -X POST -d'{}' 100.73.94.71:8080
```