package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

const honeycombEndpoint = "https://api.honeycomb.io/1/events/morton_meters_v2"

type rtlamr_record struct {
	Time    string `json:"Time"`
	Offset  int    `json:"Offset"`
	Length  int    `json:"Length"`
	Type    string `json:"Type"`
	Message struct {
		ID          int `json:"ID"`
		Type        int `json:"Type"`
		TamperPhy   int `json:"TamperPhy"`
		TamperEnc   int `json:"TamperEnc"`
		Consumption int `json:"Consumption"`
		ChecksumVal int `json:"ChecksumVal"`
	} `json:"Message"`
}

type honeycomb_record struct {
	ID   int `json:"ID"`
	Type int `json:"Type"`
	Rate int `json:"Rate"`
}

func main() {
	token := os.Getenv("HONEYCOMB_TOKEN")
	if token == "" {
		log.Fatal("must have HONEYCOMB_TOKEN set")
	}

	dataset := os.Getenv("HONEYCOMB_DATASET")
	if dataset == "" {
		log.Fatal("must have HONEYCOMB_DATASET set")
	}

	readings := make(map[int]int)

	client := &http.Client{}

	var reading rtlamr_record

	dec := json.NewDecoder(os.Stdin)
	for {
		err := dec.Decode(&reading)
		if err == io.EOF {
			return
		}
		if err != nil {
			log.Fatalln(err)
		}

		var rate int
		prev, seen := readings[reading.Message.ID]
		readings[reading.Message.ID] = reading.Message.Consumption
		if seen {
			rate = reading.Message.Consumption - prev
			fmt.Printf("seen %d before, rate in interim was %d\n", reading.Message.ID, rate)
		} else {
			fmt.Printf("First reading for %d\n", reading.Message.ID)
		}

		message, err := json.Marshal(honeycomb_record{
			ID:   reading.Message.ID,
			Type: reading.Message.Type,
			Rate: rate,
		})
		if err != nil {
			log.Fatalln(err)
		}

		req, err := http.NewRequest("POST", honeycombEndpoint, bytes.NewReader(message))
		if err != nil {
			log.Fatalln(err)
		}
		req.Header.Add("X-Honeycomb-Team", token)
		req.Header.Add("X-Honeycomb-Event-Time", reading.Time)

		resp, err := client.Do(req)
		if err != nil {
			log.Fatalln(err)
		}
		if resp.StatusCode != 200 {
			log.Fatalln("honeycomb responded not 200")
		}
		resp.Body.Close()
	}
}
