package main

import (
	"os"
	"strings"
)

type point struct {
    x int
    y int
}

func sol1(lines [][]byte) {
    xmas := []byte("XMAS")

    var res int
    vectors := []point{{1, 0}, {-1, 0}, {0, 1}, {0, -1}, {1, 1}, {1, -1}, {-1, 1}, {-1, -1}}

    for y := range len(lines) {
        for x := range len(lines[y]) {
            if lines[y][x] == xmas[0] {
                for _, vector := range vectors {
                    check := point{x, y}
                    var found bool = true
                    for _,  character := range xmas[1:] {
                        check.x = check.x + vector.x
                        check.y = check.y + vector.y
                        if check.x < 0 || check.x >= len(lines[0]) ||
                            check.y < 0 || check.y >= len(lines) {
                            found = false
                            break
                        }

                        if lines[check.y][check.x] != character {
                            found = false
                            break
                        }
                    }
                    if found {
                        res = res + 1
                    }
                }
            }
        }
    }
    println(res)
}

func sol2(lines [][]byte) {
    mas := []byte("MAS")
    var res int
    vectors := []point{{1, 1}, {1, -1}, {-1, 1}, {-1, -1}}
    a_map := make(map[point]int)

    for y := range len(lines) {
        for x := range len(lines[y]) {
            if lines[y][x] == mas[0] {
                for _, vector := range vectors {
                    check := point{x, y}
                    var a_point point
                    var found bool = true
                    for _,  character := range mas[1:] {
                        check.x = check.x + vector.x
                        check.y = check.y + vector.y
                        if check.x < 0 || check.x >= len(lines[0]) ||
                            check.y < 0 || check.y >= len(lines) {
                            found = false
                            break
                        }

                        if lines[check.y][check.x] != character {
                            found = false
                            break
                        }
                        if lines[check.y][check.x] == mas[1] {
                            a_point = check
                        }
                    }
                    if found {
                        count := a_map[a_point]
                        count++
                        a_map[a_point] = count
                    }
                }
            }
        }
    }

    for _, count := range a_map {
        if count == 2 {
            res = res + 1
        }
    }
    println(res)

}

func main() {
    contents, _ := os.ReadFile("input_2")

    var lines [][]byte
    for _, line := range strings.Split(string(contents), "\n") {
        if len(line) > 0 {
            lines = append(lines, []byte(line))
        }
    }

    sol1(lines)
    sol2(lines)
}

