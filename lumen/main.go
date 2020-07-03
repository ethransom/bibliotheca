package main

import (
	"flag"
	"log"
	"net/http"
)

var addr = flag.String("addr", "localhost:8080", "http service address")

func main() {
	flag.Parse()
	fs := http.FileServer(http.Dir("static"))
	http.Handle("/", fs)
	log.Fatal(http.ListenAndServe(*addr, nil))
}
