package main

import (
    "fmt"
    "os"
    "strconv"
    "strings"
)

type operator int
const (
    add operator = iota
    multiply
    concat
)

func getOperators() []operator {
    return []operator{add, multiply, concat}
}

func parseInput(contents *[]byte) map[int][]int {
    equations := make(map[int][]int)
    for _, line := range strings.Split(strings.TrimSpace(string(*contents)), "\n") {
        parts := strings.Split(line, ":")
        var eq []int
        for _, p := range strings.Split(strings.TrimSpace(parts[1]), " ") {
            val, _ := strconv.Atoi(p)
            eq = append(eq, val)
        }
        val, _ := strconv.Atoi(parts[0])
        equations[val] = eq
    }

    return equations
}

func sol(contents *[]byte) {
    equations := parseInput(contents)

    var res int
    for key, values := range equations {
        if sum(0, values, key) {
            res += key
        }
    }
    fmt.Println(res)
}

func main() {
    contents, _ := os.ReadFile("./input_2")
    sol(&contents)
}

func sum(current int, remaining []int, key int) bool {
    var res bool
    if len(remaining) == 0 {
        if current == key {
            return true
        }
        return false
    }
    for _, oper := range getOperators() {
        new := current
        switch oper {
        case add:
            new += remaining[0]
        case multiply:
            new *= remaining[0]
        case concat:
            str_curr := strconv.Itoa(current)
            str_new  := strconv.Itoa(remaining[0])
            new, _ = strconv.Atoi(str_curr+str_new)
        }
        if new <= key {
            res = sum(new, remaining[1:], key)
        }
        if res { break }
    }
    return res
}
