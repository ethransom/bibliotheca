package main

import (
	"fmt"
)

func solve(nums []int, length int) int {
	placeholders := make(map[int]int)
	for i, n := range nums {
		placeholders[n] = i
	}

	for i := len(nums); i < length; i++ {
		target := nums[i-1]
		diff := 0
		if last, ok := placeholders[target]; ok {
			diff = i - 1 - last
		}

		nums = append(nums, diff)
		placeholders[target] = i - 1
	}

	return nums[len(nums)-1]
}

func main() {
	inputs := [][]int{
		{0, 3, 6},
		{1, 3, 2},
		{2, 1, 3},
		{1, 2, 3},
		{2, 3, 1},
		{3, 2, 1},
		{3, 1, 2},
		{15, 5, 1, 4, 7, 0},
	}
	for _, input := range inputs {
		fmt.Printf("Given %v:\n", input)

		fmt.Printf("\t      2020th spoken is %d\n", solve(input, 2020))
		fmt.Printf("\t30_000_000th spoken is %d\n", solve(input, 30_000_000))

	}
}
