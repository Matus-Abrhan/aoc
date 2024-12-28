package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func transform(stones map[int]int) map[int]int {
    new_stones := make(map[int]int)
    for stone, count := range stones {
        if stone == 0 {
            new_stones[1] += count
        } else if str := strconv.Itoa(stone); len(str)%2 == 0 {
            stone1, _ := strconv.Atoi(str[:len(str)/2])
            stone2, _ := strconv.Atoi(str[len(str)/2:])
            new_stones[stone1] += count
            new_stones[stone2] += count
        } else {
            new_stones[stone*2024] += count
        }
    }

    return new_stones
}

func parseInput(contents *[]byte) map[int]int {
    stones := make(map[int]int)
    lines := strings.Split(strings.TrimSpace(string(*contents)), "\n")
    for _, line := range lines {
        for _, stone := range strings.Split(line, " ") {
            s, _ := strconv.Atoi(stone)
            prev := stones[s]
            stones[s] = prev+1
        }
    }

    return stones
}

func sol(contents *[]byte) {
    stones := parseInput(contents)

    for i:=0; i<75; i++ {
        stones = transform(stones)
    }

    var res int
    for _, count := range stones {
        res += count
    }
    fmt.Println(res)
}

func main() {
    contents, _ := os.ReadFile("input_2")
    sol(&contents)
}
