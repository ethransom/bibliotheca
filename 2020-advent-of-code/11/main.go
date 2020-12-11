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

type neighborAlgorithm func(rows [][]rune, r int, c int) (neighbors int)

func print(rows [][]rune) {
	for _, row := range rows {
		fmt.Printf("%s\n", string(row))
	}
}

func simulate(rows [][]rune, algo neighborAlgorithm, threshold int) ([][]rune, bool) {
	var dirty bool

	nextRows := make([][]rune, len(rows))
	for r, _ := range rows {
		nextRows[r] = make([]rune, len(rows[r]))
		for c, _ := range rows[r] {
			if rows[r][c] == 'L' && algo(rows, r, c) == 0 {
				nextRows[r][c] = '#'
				dirty = true
			} else if rows[r][c] == '#' && algo(rows, r, c) >= threshold {
				nextRows[r][c] = 'L'
				dirty = true
			} else {
				nextRows[r][c] = rows[r][c]
			}
		}
	}

	return nextRows, dirty
}

func eightDirections() [][]int {
	return [][]int{{-1, -1}, {-1, 0}, {-1, 1}, {0, 1}, {1, 1}, {1, 0}, {1, -1}, {0, -1}}
}

func neighborsByAdjacency(rows [][]rune, r int, c int) (neighbors int) {
	height, width := len(rows), len(rows[0])

	for _, dir := range eightDirections() {
		i, j := dir[0], dir[1]

		if r+i >= 0 && c+j >= 0 && r+i < height && c+j < width {
			if rows[r+i][c+j] == '#' {
				neighbors += 1
			}
		}
	}

	return
}

func neighborsByLineOfSight(rows [][]rune, r int, c int) (neighbors int) {
	height, width := len(rows), len(rows[0])

	for _, dir := range eightDirections() {
		i, j := r, c
		dy, dx := dir[0], dir[1]

		for {
			i += dy
			j += dx

			if i < 0 || j < 0 || i >= height || j >= width {
				break
			}

			if rows[i][j] == 'L' {
				break
			}

			if rows[i][j] == '#' {
				neighbors += 1
				break
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

func solve(filename string, algo neighborAlgorithm, threshold int) (int, int) {
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

		rows, dirty = simulate(rows, algo, threshold)
	}

	return occupancy(rows), runs
}

func main() {
	files := []string{"example.txt", "input.txt"}

	for _, file := range files {
		occupancy, runs := solve(file, neighborsByAdjacency, 4)
		fmt.Printf("adjacency:     %d after %d runs\n", occupancy, runs)

		occupancy, runs = solve(file, neighborsByLineOfSight, 5)
		fmt.Printf("line-of-sight: %d after %d runs\n", occupancy, runs)
	}
}
