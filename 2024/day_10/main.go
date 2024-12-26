package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type point struct {x, y int}

func getDirections() []point {
    return []point{{0, 1}, {1, 0}, {-1, 0}, {0, -1}}
}

func parseInput(contents *[]byte) ([]point, map[point]int) {
    var heads []point
    points := make(map[point]int)

    lines := strings.Split(strings.TrimSpace(string(*contents)), "\n")
    for y, line := range lines {
        for x, char := range line {
            switch char {
            case '0':
                heads = append(heads, point{x, y})
            default:
                points[point{x, y}], _ = strconv.Atoi(string(char))
            }
        }
    }

    return heads, points
}

func sol1(contents *[]byte) {
    heads, points := parseInput(contents)

    var res int
    for j:=0; j<len(heads); j++ {
        positions := heads[j:j+1]
        for i:=1; i<10; i++ {
            var new_positions []point
            for _, pos := range positions {
                for _, dir := range getDirections() {
                    new_pos := point{pos.x+dir.x, pos.y+dir.y}
                    if points[new_pos] == i {
                        new_positions = append(new_positions, new_pos)
                    }
                }
            }
            positions = new_positions
        }
        unique := make(map[point]bool)
        for _, pos := range positions {
            unique[pos] = true
        }
        res += len(unique)
    }
    fmt.Println(res)
}

func sol2(contents *[]byte) {
    heads, points := parseInput(contents)

    var res int
    for j:=0; j<len(heads); j++ {
        positions := heads[j:j+1]
        for i:=1; i<10; i++ {
            var new_positions []point
            for _, pos := range positions {
                for _, dir := range getDirections() {
                    new_pos := point{pos.x+dir.x, pos.y+dir.y}
                    if points[new_pos] == i {
                        new_positions = append(new_positions, new_pos)
                    }
                }
            }
            positions = new_positions
        }
        res += len(positions)
    }
    fmt.Println(res)
}

func main() {
    contents, _ := os.ReadFile("input_2")
    sol1(&contents)
    sol2(&contents)
}
