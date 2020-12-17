package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type grid struct {
	pages  int
	rows   int
	cells  int
	values []bool
}

func newGrid(pages, rows, cells int) grid {
	return grid{
		pages:  pages,
		rows:   rows,
		cells:  cells,
		values: make([]bool, pages*rows*cells),
	}
}

func newGridFrom(g grid) grid {
	return grid{
		pages:  g.pages,
		rows:   g.rows,
		cells:  g.cells,
		values: make([]bool, g.pages*g.rows*g.cells),
	}
}

func (g *grid) set(page, row, cell int, value bool) {
	g.values[(page*g.rows*g.cells)+(row*g.cells)+cell] = value
}

func (g *grid) get(page, row, cell int) bool {
	return g.values[(page*g.rows*g.cells)+(row*g.cells)+cell]
}

func (g *grid) print() {
	for page := 0; page < len(g.values)/(g.rows*g.cells); page++ {
		fmt.Printf("z=%d\n", page)
		for row := 0; row < g.rows; row++ {
			for cell := 0; cell < g.cells; cell++ {
				if g.get(page, row, cell) {
					fmt.Print("#")
				} else {
					fmt.Print(".")
				}
			}
			fmt.Println()
		}
		fmt.Println()
	}
}

func (g grid) count() (c int) {
	for _, cell := range g.values {
		if cell {
			c++
		}
	}

	return
}

func offsets() [][]int {
	out := make([][]int, 0, 26)
	for i := -1; i < 2; i++ {
		for j := -1; j < 2; j++ {
			for k := -1; k < 2; k++ {
				if i == 0 && j == 0 && k == 0 {
					continue
				}
				out = append(out, []int{i, j, k})
			}
		}
	}
	return out
}

func (g grid) neighborCount(page, row, cell int) (neighbors int) {
	for _, n := range offsets() {
		p, r, c := page+n[0], row+n[1], cell+n[2]

		if p >= 0 && r >= 0 && c >= 0 && p < g.pages && r < g.rows && c < g.cells {
			if g.get(p, r, c) {
				neighbors++
			}
		}
	}

	return
}

func solve(input [][]rune, iters int) int {
	depth, height, width := iters*2, len(input)+iters*2, len(input[0])+iters*2
	grid := newGrid(depth, height, width)
	for row := range input {
		for cell := range input[row] {
			if input[row][cell] == '#' {
				grid.set(
					depth/2,
					height/2-len(input)/2+row,
					width/2-len(input[0])/2+cell,
					true,
				)
			}
		}
	}

	for i := 0; i < iters; i++ {
		nextGrid := newGridFrom(grid)
		for page := 0; page < grid.pages; page++ {
			for row := 0; row < grid.rows; row++ {
				for cell := 0; cell < grid.cells; cell++ {
					if grid.get(page, row, cell) {
						if c := grid.neighborCount(page, row, cell); c == 2 || c == 3 {
							nextGrid.set(page, row, cell, true)
						} else {
							nextGrid.set(page, row, cell, false)
						}
					} else {
						if grid.neighborCount(page, row, cell) == 3 {
							nextGrid.set(page, row, cell, true)
						} else {
							nextGrid.set(page, row, cell, false)
						}
					}
				}
			}
		}
		grid = nextGrid
	}

	// grid.print()

	return grid.count()
}

func parse(filename string, threshold int) (rows [][]rune) {
	dat, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(dat), "\n")

	rows = make([][]rune, len(lines))
	for r := range lines {
		rows[r] = []rune(lines[r])
	}

	return
}

const runs = 6

func main() {
	files := []string{
		"example.txt",
		"input.txt",
	}

	for _, file := range files {
		fmt.Printf("%v:\n", file)

		input := parse(file, 4)

		occupancy := solve(input, runs)

		fmt.Printf("\t%d after %d runs\n", occupancy, runs)
	}
}
