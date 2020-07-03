GOOS=linux go build -o build/linux-yeet

docker build -t gcr.io/personal-147902/yeet .

docker push gcr.io/personal-147902/yeet