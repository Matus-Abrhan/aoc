package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
    down = -1
    up = 1

    threshold = 1
)

func check_report(line string, ) int {
    var direction, prev, dampener int

    for _, level := range strings.Split(line, " ") {
        current, _ := strconv.Atoi(level)
        if direction == 0 {
            if prev == 0 {
                prev = current
                continue
            }
            diff := current - prev
            if diff > 0 {
                direction = up
            } else if diff < 0 {
                direction = down
            } else {
                dampener++
                // return 0
            }
        }

        diff := (current - prev) * direction
        if diff < 1 || diff > 3 {
            dampener++
            if dampener > threshold {
                return 0
            }
        }

        prev = current
    }

    return 1
}

func main() {
    content, _ := os.ReadFile("./input_2")

    var res int
    for _, line := range strings.Split(string(content), "\n") {
        if len(line) > 0 {
            res = res + check_report(line)
        }
    }

    fmt.Println(res)
}
