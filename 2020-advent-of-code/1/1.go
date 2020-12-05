package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	dat, err := ioutil.ReadFile("input.txt")
	check(err)

	lines := strings.Split(string(dat), "\n")

	var nums []int64
	for _, line := range lines {
		num, err := strconv.ParseInt(line, 0, 64)
		if err == nil {
			nums = append(nums, num)
		}
	}

	for i := 0; i < (len(nums) - 1); i++ {
		for j := i; j < len(nums); j++ {
			if nums[i]+nums[j] == 2020 {
				fmt.Println(nums[i] * nums[j])
				return
			}
		}
	}
}
