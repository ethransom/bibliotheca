GOOS=linux go build -o build/linux-lumen

docker build -t gcr.io/personal-147902/lumen .

docker push gcr.io/personal-147902/lumen