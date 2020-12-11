package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

func check(nums []int, window int, pos int) bool {
	// TODO: slices????
	// fmt.Printf("checking %d for any previous pair in:\n", nums[pos])
	for i := pos - window; i < pos-1; i++ {
		// fmt.Printf("\t%d\n", nums[i])
		for j := i + 1; j < pos; j++ {
			// fmt.Printf("\t\t%d <> %d\n", nums[i], nums[j])
			if nums[i]+nums[j] == nums[pos] {
				return true
			}
		}
	}

	return false
}

func findInvalid(nums []int, window int) int {
	for pos := window; pos < len(nums); pos++ {
		num := nums[pos]
		if !check(nums, window, pos) {
			return num
		}
	}

	return -1
}

func findRange(nums []int, target int) []int {
	for i := 0; i < len(nums)-1; i++ {
		sum := nums[i]
		for j := i + 1; j < len(nums); j++ {
			sum += nums[j]
			if sum == target {
				return nums[i : j+1]
			}
		}
	}

	return []int{}
}

func parse(filename string) ([]int, error) {
	dat, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(dat), "\n")

	nums := make([]int, len(lines))
	for i, line := range lines {
		num, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}

		nums[i] = num
	}

	return nums, nil
}

func main() {
	inputs := []struct {
		filename string
		window   int
	}{{"example.txt", 5}, {"input.txt", 25}}

	for _, input := range inputs {
		nums, err := parse(input.filename)
		if err != nil {
			panic(err)
		}

		fmt.Printf("%s:\n", input.filename)

		invalid := findInvalid(nums, input.window)

		fmt.Printf("\t1: (invalid num): %d\n", invalid)

		set := findRange(nums, invalid)

		sort.Ints(set)

		fmt.Printf("\t2:       (range): %d\n", set[0]+set[len(set)-1])
	}
}
