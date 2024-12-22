package main

import (
	"fmt"
	"os"
	"strings"
)

type point struct { x, y int }

func parseInput(contents *[]byte) (map[rune][]point, point) {
    antenas_map := make(map[rune][]point)
    lines := strings.Split(strings.TrimSpace(string(*contents)), "\n")
    size := point{len(lines[0]), len(lines)}
    for y, line := range lines {
        for x, char := range line {
            if char != '.' {
                freq := antenas_map[char]
                freq = append(freq, point{x, y})
                antenas_map[char] = freq
            }
        }
    }
    return antenas_map, size
}

func sol1(contents *[]byte) {
    anthenas_map, size := parseInput(contents)
    res := make(map[point]bool)
    for _, anthenas := range anthenas_map {
        for i, anth_a := range anthenas[:len(anthenas)-1] {
            for _, anth_b := range anthenas[i+1:] {
                dist := point{anth_b.x-anth_a.x, anth_b.y-anth_a.y}
                p1 := point{anth_a.x-dist.x, anth_a.y-dist.y}
                p2 := point{anth_b.x+dist.x, anth_b.y+dist.y}

                if isInBounds(p1, size) { res[p1]=true }
                if isInBounds(p2, size) { res[p2]=true }
            }
        }
    }
    fmt.Println(len(res))
}

func isInBounds(p point, size point) bool {
    if p.x < 0 || p.x >= size.x ||
    p.y < 0 || p.y >= size.y {
        return false
    }
    return true
}

func sol2(contents *[]byte) {
    anthenas_map, size := parseInput(contents)
    res := make(map[point]bool)
    for _, anthenas := range anthenas_map {
        for _, anth_a := range anthenas {
            for _, anth_b := range anthenas {
                if anth_a == anth_b {continue}
                dist := point{anth_b.x-anth_a.x, anth_b.y-anth_a.y}

                idx := 0
                for true {
                    p := point{anth_a.x-(dist.x*idx), anth_a.y-(dist.y*idx)}
                    if !isInBounds(p, size) {
                        break
                    }
                    res[p] = true
                    idx++
                }
            }
        }
    }
    fmt.Println(len(res))
}

func main() {
    contents, _ := os.ReadFile("./input_2")
    sol1(&contents)
    sol2(&contents)
}
