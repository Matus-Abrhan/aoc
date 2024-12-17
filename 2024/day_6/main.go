package main

import (
	"fmt"
	"os"
	"strings"
)

type point struct {x, y int}
type guard struct {pos, dir point}

func getDirections() []point {
    return []point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}
}

func getRoomEdge(g guard, size point) int {
    var res int
    switch g.dir{
        case point{-1, 0}, point{0, -1}:
            res = -1*((g.pos.x*g.dir.x) + (g.pos.y*g.dir.y))
            res+=1
        case point{1, 0}, point{0, 1}:
            p := point{size.x-g.pos.x, size.y-g.pos.y}
            res = (p.x*g.dir.x) + (p.y*g.dir.y)
    }
    res++
    return res
}

func sign(a point) point {
    var new_point point 
    switch {
    case a.x < 0:
        new_point.x = -1
    case a.x > 0:
        new_point.x = 1
    }
    switch {
    case a.y < 0:
        new_point.y = -1
    case a.y > 0:
        new_point.y = 1
    }
    return new_point
}

func (g *guard) turn(n int) {
    directions := getDirections()
    for idx, dir := range directions {
        if g.dir == dir {
            new_idx := (idx+n+len(directions)) % len(directions)
            g.dir = directions[new_idx]
            break
        }
    }
}

func (g *guard) move(n int) {
    g.pos.x += g.dir.x*n
    g.pos.y += g.dir.y*n
}

func isInBounds(pos point, size point) bool {
    if pos.x<0 || pos.x>=size.x ||
        pos.y<0 || pos.y>=size.y {
        return false
    }
    return true
}

func parseInput(contents *[]byte) (guard, []point, point) {
    var o []point
    var g guard

    lines := strings.Fields(string(*contents))
    for y, line := range lines {
        if len(line) <= 0 { continue }
        for x, byte := range line {
            switch string(byte) {
            case "#":
                o = append(o, point{x, y})
            case "^":
                g.pos= point{x, y}
                g.dir= point{0, -1}
            case ">":
                g.pos= point{x, y}
                g.dir= point{1, 0}
            case "V":
                g.pos= point{x, y}
                g.dir= point{0, 1}
            case "<":
                g.pos= point{x, y}
                g.dir= point{-1, 0}
            }
        }
    }

    s := point{len(lines[0]), len(lines)}
    return g, o, s
}

func sol(contents []byte) {
    g, obstacles, room_size := parseInput(&contents)

    visited := make(map[point]bool)
    var path []point
    for isInBounds(g.pos, room_size) {
        visited[g.pos] = true
        path = append(path, g.pos)
        next_pos := point{g.pos.x+g.dir.x, g.pos.y+g.dir.y}

        for _, obstacle := range obstacles {
            if obstacle == next_pos {
                g.turn(1)
                next_pos = g.pos
                break
            }
        }
        g.pos = next_pos
    }
    fmt.Println(len(visited))

    g, obstacles, room_size = parseInput(&contents)
    new_obstacles := make(map[point]bool)
    for idx:=1; idx<len(path); idx++ {
        g2 := g
        obstacles = append(obstacles, path[idx])

        loop := make(map[guard]bool)
        for isInBounds(g2.pos, room_size) {
            if loop[g2] {
                new_obstacles[path[idx]] = true
                break
            }
            loop[g2] = true

            room_edge := getRoomEdge(g2, room_size)
            min_dist := room_edge
            for _, obstacle := range obstacles {
                vec := point{obstacle.x-g2.pos.x, obstacle.y-g2.pos.y}
                if sign(vec) == g2.dir {
                    dist := (vec.x*g2.dir.x) + (vec.y*g2.dir.y)
                    min_dist = min(min_dist, dist)
                }
            }
            g2.move(min_dist-1)
            g2.turn(1)

        }
        obstacles = obstacles[:len(obstacles)-1]
    }
    fmt.Println(len(new_obstacles))
}

func main() {
    contents, _ := os.ReadFile("./input_2")
    sol(contents)
}
