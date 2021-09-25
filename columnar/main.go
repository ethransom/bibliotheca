package main

import (
	"fmt"
	"math/rand"
)

func main() {
	intIndex := make(map[int64]int64)
	strIndex := make(map[string][]int64) // SCUBA would be much better :(

	values := []string{"pending", "success", "error", "invalid", "dropped"}
	r := rand.New(rand.NewSource(99))
	for i := 0; i < 1_000_000; i++ {
		id := int64(i)

		intVal := r.Int63()
		intIndex[intVal] = id

		strVal := values[r.Int()%len(values)]
		bucket, ok := strIndex[strVal]
		if !ok {
			strIndex[strVal] = make([]int64, 1)
		}
		strIndex[strVal] = append(bucket, id)

	}

	// COUNT where int between values
	ids := make([]int64, 0)
	for k, v := range intIndex {
		if k > 1601398729736864047 && k < 4869713468087973210 {
			ids = append(ids, v)
		}
	}
	fmt.Printf("found %d matches\n", len(ids))

	// COUNT where str is value
	// idk, surely this is cheating
	ids = strIndex["error"]
	fmt.Printf("found %d matches\n", len(ids))

	// TODO: compound queries:
	// COUNT, GROUP BY str, where int between
}
