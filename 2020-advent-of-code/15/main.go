package main

import (
	"fmt"
)

func solve(nums []int, length int) int {
	for i := len(nums); i < length; i++ {
		target := nums[i-1]
		diff := 0
		for j := i - 2; j >= 0; j-- {
			if nums[j] == target {
				diff = i - 1 - j
				break
			}
		}

		nums = append(nums, diff)
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
		soln := solve(input, 30000000)

		fmt.Printf("%v: %d\n", input, soln)

	}
}
