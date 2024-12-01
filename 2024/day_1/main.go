package main

import (
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func sol1() {
    contents, _ := os.ReadFile("input_2")

    var left, right []int
    for _, line := range strings.Split(string(contents), "\n") {
	line_parts := strings.SplitN(line, " ", 2)
	new_left, err := strconv.Atoi(strings.TrimSpace(line_parts[0]))
	if err != nil { continue}
	new_right, err := strconv.Atoi(strings.TrimSpace(line_parts[1]))
	if err != nil { continue}

	left = append(left, new_left)
	right = append(right, new_right)
    }

    sort.Sort(sort.IntSlice(left))
    sort.Sort(sort.IntSlice(right))

    var res int = 0;
    for idx, val1 := range left {
	res += int(math.Abs(float64(val1-right[idx])))
    }
    fmt.Println(res)
}

func sol2() {
    contents, _ := os.ReadFile("input_2")

    var left []int
    right := make(map[int]int)
    for _, line := range strings.Split(string(contents), "\n") {
	line_parts := strings.SplitN(line, " ", 2)
	new_left, err := strconv.Atoi(strings.TrimSpace(line_parts[0]))
	if err != nil {continue}
	new_right, err := strconv.Atoi(strings.TrimSpace(line_parts[1]))
	if err != nil {continue}

	left = append(left, new_left)
	right[new_right] += 1
    }

    var res int = 0;
    for _, val1 := range left {
	res += val1*right[val1]
    }
    fmt.Println(res)
}


func main() {
    // sol1()
    sol2()
}
