package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"

	_ "github.com/lib/pq"
)

const dbInfo = "host=localhost port=5432 user=postgres dbname=meter_readings sslmode=disable"

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

func main() {
	db, err := sql.Open("postgres", dbInfo)
	if err != nil {
		log.Fatal(err)
	}
	http.HandleFunc("/query", makeRetriever(db))
	http.HandleFunc("/", makeReceiver(db))
	http.ListenAndServe(":8080", nil)
}

func makeReceiver(db *sql.DB) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		body, err := ioutil.ReadAll(r.Body)
		if err != nil {
			log.Println("error reading request", err)
			return
		}
		log.Println("message:", string(body))
		var reading rtlamr_record
		err = json.Unmarshal(body, &reading)
		if err != nil {
			log.Println("error parsing json", err)
			return
		}
		log.Printf("%d @ %d on %s\n", reading.Message.ID, reading.Message.Consumption, reading.Time)
		err = insertReadingQuery(db, reading)
		if err != nil {
			log.Println("error inserting into db", err)
			return
		}
	}
}

func makeRetriever(db *sql.DB) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		err := readQuery(db, powerTimeseriesQuery, w)
		if err != nil {
			log.Println("error reading from db", err)
			return
		}
	}
}

func insertReadingQuery(db *sql.DB, reading rtlamr_record) (err error) {
	query := "INSERT INTO readings (time, id, type, consumption) VALUES ($1, $2, $3, $4)"

	_, err = db.Exec(
		query,
		reading.Time,
		reading.Message.ID,
		reading.Message.Type,
		reading.Message.Consumption,
	)

	return
}

const powerTimeseriesQuery = `
with intervals as (
	select 
		min(time)::timestamptz at time zone 'America/Denver' as start_time,
		EXTRACT(epoch from min(time)) as start_time_unix,
		consumption * 10 as watt_hours
	from readings 
	group by consumption
)
select 
	start_time,
	start_time_unix,
	(watt_hours - lag(watt_hours, 2) over (order by start_time asc))
	/ 
	((start_time_unix - lag(start_time_unix, 2) over (order by start_time asc)) / 3600) as avg_watts_in_interval
from intervals
order by start_time desc
`

func readQuery(db *sql.DB, query string, w io.Writer) (err error) {
	jsonQuery := fmt.Sprintf("select json_agg(t) from (%s) as t", query)

	var result string
	row := db.QueryRow(jsonQuery)
	err = row.Scan(&result)
	switch err {
	case sql.ErrNoRows:
		fmt.Fprintln(w, "No rows were returned!")
		return
	case nil:
		fmt.Fprintln(w, result)
	}

	return
}
