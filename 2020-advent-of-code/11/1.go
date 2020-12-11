package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func print(rows [][]rune) {
	for _, row := range rows {
		fmt.Printf("%s\n", string(row))
	}
}

func simulate(rows [][]rune) ([][]rune, bool) {
	var dirty bool

	nextRows := make([][]rune, len(rows))
	for r, _ := range rows {
		nextRows[r] = make([]rune, len(rows[r]))
		for c, _ := range rows[r] {
			if rows[r][c] == 'L' && neighbors(rows, r, c) == 0 {
				nextRows[r][c] = '#'
				dirty = true
			} else if rows[r][c] == '#' && neighbors(rows, r, c) >= 4 {
				nextRows[r][c] = 'L'
				dirty = true
			} else {
				nextRows[r][c] = rows[r][c]
			}
		}
	}

	return nextRows, dirty
}

func neighbors(rows [][]rune, r int, c int) (neighbors int) {
	height, width := len(rows), len(rows[0])

	for i := -1; i != 2; i++ {
		for j := -1; j != 2; j++ {
			if i == 0 && j == 0 {
				continue
			}
			if r+i >= 0 && c+j >= 0 && r+i < height && c+j < width {
				if rows[r+i][c+j] == '#' {
					neighbors += 1
				}
			}
		}
	}

	return
}

func occupancy(rows [][]rune) (count int) {
	for _, row := range rows {
		for _, cell := range row {
			if cell == '#' {
				count += 1
			}
		}
	}

	return
}

func solve(filename string) {
	dat, err := ioutil.ReadFile(filename)
	check(err)

	lines := strings.Split(string(dat), "\n")

	rows := make([][]rune, len(lines))
	for r, _ := range lines {
		rows[r] = []rune(lines[r])
	}

	dirty := true
	runs := 0
	for dirty && runs < 10000 {
		runs += 1

		rows, dirty = simulate(rows)

		// print(rows)
	}

	fmt.Printf("occupancy %d after %d runs\n", occupancy(rows), runs)
}

func main() {
	files := []string{"example.txt", "input.txt"}

	for _, file := range files {
		solve(file)
	}
}
