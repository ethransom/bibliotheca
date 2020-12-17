package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type grid3 struct {
	pages  int
	rows   int
	cells  int
	values []bool
}

func newGrid3(pages, rows, cells int) grid3 {
	return grid3{
		pages:  pages,
		rows:   rows,
		cells:  cells,
		values: make([]bool, pages*rows*cells),
	}
}

func newGrid3From(g grid3) grid3 {
	return grid3{
		pages:  g.pages,
		rows:   g.rows,
		cells:  g.cells,
		values: make([]bool, len(g.values)),
	}
}

func (g *grid3) set(page, row, cell int, value bool) {
	g.values[(page*g.rows*g.cells)+(row*g.cells)+cell] = value
}

func (g *grid3) get(page, row, cell int) bool {
	return g.values[(page*g.rows*g.cells)+(row*g.cells)+cell]
}

func (g *grid3) print() {
	for page := 0; page < g.pages; page++ {
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

func (g grid3) count() (c int) {
	for _, cell := range g.values {
		if cell {
			c++
		}
	}

	return
}

func offsets3() [][]int {
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

func (g grid3) neighborCount(page, row, cell int) (neighbors int) {
	for _, n := range offsets3() {
		p, r, c := page+n[0], row+n[1], cell+n[2]

		if p >= 0 && r >= 0 && c >= 0 && p < g.pages && r < g.rows && c < g.cells {
			if g.get(p, r, c) {
				neighbors++
			}
		}
	}

	return
}

func solve3(input [][]rune, iters int) int {
	depth, height, width := iters*2, len(input)+iters*2, len(input[0])+iters*2
	grid := newGrid3(depth, height, width)
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
		nextGrid := newGrid3From(grid)
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

type grid4 struct {
	files  int
	pages  int
	rows   int
	cells  int
	values []bool
}

func newGrid4(files, pages, rows, cells int) grid4 {
	return grid4{
		files:  files,
		pages:  pages,
		rows:   rows,
		cells:  cells,
		values: make([]bool, files*pages*rows*cells),
	}
}

func newGrid4From(g grid4) grid4 {
	return grid4{
		files:  g.files,
		pages:  g.pages,
		rows:   g.rows,
		cells:  g.cells,
		values: make([]bool, len(g.values)),
	}
}

func (g *grid4) set(file, page, row, cell int, value bool) {
	g.values[(file*g.pages*g.rows*g.cells)+(page*g.rows*g.cells)+(row*g.cells)+cell] = value
}

func (g *grid4) get(file, page, row, cell int) bool {
	return g.values[(file*g.pages*g.rows*g.cells)+(page*g.rows*g.cells)+(row*g.cells)+cell]
}

func (g *grid4) print() {
	panic("not implemented")

	for file := 0; file < g.files; file++ {
		for page := 0; page < g.pages; page++ {
			fmt.Printf("z=%d\n", page)
			for row := 0; row < g.rows; row++ {
				for cell := 0; cell < g.cells; cell++ {
					if g.get(file, page, row, cell) {
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
}

func (g grid4) count() (c int) {
	for _, cell := range g.values {
		if cell {
			c++
		}
	}

	return
}

func offsets4() [][]int {
	out := make([][]int, 0)
	for i := -1; i < 2; i++ {
		for j := -1; j < 2; j++ {
			for k := -1; k < 2; k++ {
				for l := -1; l < 2; l++ {
					if i == 0 && j == 0 && k == 0 && l == 0 {
						continue
					}
					out = append(out, []int{i, j, k, l})
				}
			}
		}
	}
	return out
}

func (g grid4) neighborCount(file, page, row, cell int) (neighbors int) {
	for _, n := range offsets4() {
		f, p, r, c := file+n[0], page+n[1], row+n[2], cell+n[3]

		if f >= 0 && p >= 0 && r >= 0 && c >= 0 && f < g.files && p < g.pages && r < g.rows && c < g.cells {
			if g.get(f, p, r, c) {
				neighbors++
			}
		}
	}

	return
}

func solve4(input [][]rune, iters int) int {
	foom, depth, height, width := iters*4, iters*4, len(input)+iters*4, len(input[0])+iters*4
	grid := newGrid4(foom, depth, height, width)
	for row := range input {
		for cell := range input[row] {
			if input[row][cell] == '#' {
				grid.set(
					foom/2,
					depth/2,
					height/2-len(input)/2+row,
					width/2-len(input[0])/2+cell,
					true,
				)
			}
		}
	}

	for i := 0; i < iters; i++ {
		nextGrid := newGrid4From(grid)
		for file := 0; file < grid.files; file++ {
			for page := 0; page < grid.pages; page++ {
				for row := 0; row < grid.rows; row++ {
					for cell := 0; cell < grid.cells; cell++ {
						if grid.get(file, page, row, cell) {
							if c := grid.neighborCount(file, page, row, cell); c == 2 || c == 3 {
								nextGrid.set(file, page, row, cell, true)
							} else {
								nextGrid.set(file, page, row, cell, false)
							}
						} else {
							if grid.neighborCount(file, page, row, cell) == 3 {
								nextGrid.set(file, page, row, cell, true)
							} else {
								nextGrid.set(file, page, row, cell, false)
							}
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

		fmt.Printf("\t3d: %d after %d runs\n", solve3(input, runs), runs)

		fmt.Printf("\t4d: %d after %d runs\n", solve4(input, runs), runs)
	}
}
