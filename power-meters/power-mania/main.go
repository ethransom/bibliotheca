package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	_ "github.com/mattn/go-sqlite3"
)

type rtlamrRecord struct {
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
	dsnStr := "file:test.db?cache=shared&mode=memory"
	if path, ok := os.LookupEnv("POWER_METERS_DB_PATH"); ok {
		dsnStr = fmt.Sprintf("file:%s?cache=shared&_journal=wal&mode=rwc", path)
		log.Printf("INFO using disk db at '%s'\n", path)
	} else {
		log.Println("INFO using in-memory db")
	}
	db, err := sql.Open("sqlite3", dsnStr)
	if err != nil {
		log.Fatal(err)
	}
	buildSchema(db)

	http.HandleFunc("/query", makeRetriever(db))
	http.HandleFunc("/readings", makeReceiver(db))

	http.Handle("/", http.FileServer(http.Dir("./www")))

	log.Fatal(http.ListenAndServe(":8080", nil))
}

func buildSchema(db *sql.DB) {
	log.Println("migrating db...")
	if _, err := db.Exec(`
		create table if not exists readings(
			time timestamp,
			id integer,
			type integer,
			consumption integer
		);
	`); err != nil {
		log.Fatal(err)
	}
	log.Println("migrated db")
}

func makeReceiver(db *sql.DB) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		body, err := io.ReadAll(r.Body)
		if err != nil {
			log.Println("error reading request", err)
			return
		}
		log.Println("message:", string(body))
		var reading rtlamrRecord
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
		err := readQuery(db, w)
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			_, _ = w.Write([]byte("Internal Server Error"))
			log.Println("error reading from db", err)
			return
		}
	}
}

func insertReadingQuery(db *sql.DB, reading rtlamrRecord) (err error) {
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

func readQuery(db *sql.DB, w io.Writer) (err error) {
	jsonQuery := `
			with intervals as (
				select 
					*,
					watt_hours - lag(watt_hours) over () as new_watt_hours,
					(start_time - lag(start_time) over ()) / 3600.0 as hours_since_last_update,
					(watt_hours - lag(watt_hours) over ())
					/ 
					((start_time - lag(start_time) over ()) / 3600.0) as avg_watts_in_interval
				from (
					select 
					    min(time) as start_timestamp,
						unixepoch(min(time)) as start_time,
						consumption * 10 as watt_hours
					from readings 
					-- where time >= now() - interval '1 week'
					group by consumption
				)
				order by start_time desc
				limit 1000 -- FIXME: we gotta do something more flexible
			)
			select 
			    json_group_array( 
					json_object(
						'start_time', start_time,
						'watt_hours', watt_hours,
					    'new_watt_hours', new_watt_hours,
						'hours_since_last_update', hours_since_last_update,
					    'avg_watts_in_interval', avg_watts_in_interval
					)
				)
			from intervals;
		`

	var result string
	row := db.QueryRow(jsonQuery)
	err = row.Scan(&result)
	switch err {
	case sql.ErrNoRows:
		_, err := fmt.Fprintln(w, []string{})
		return err
	case nil:
		_, err := fmt.Fprintln(w, result)
		return err
	}

	return err
}
