package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const (
    do = 1
    dont = 0
)

func sol1(contents []byte) {
    r := regexp.MustCompile("mul\\(([0-9]+),([0-9]+)\\)")
    matches := r.FindAllStringSubmatch(string(contents), -1)

    var res int
    for _, match := range matches {
        x, _ := strconv.Atoi(match[1])
        y, _ := strconv.Atoi(match[2])
        res = res + (x*y)
    }
    fmt.Println(res)
}

func sol2(contents []byte) {
    contents_str := string(contents)

    r1 := regexp.MustCompile("mul\\(([0-9]+),([0-9]+)\\)")
    mul_indexes := r1.FindAllStringIndex(contents_str, -1)

    r2 := regexp.MustCompile("(do\\(\\)|don't\\(\\))")
    cond_indexes := r2.FindAllStringIndex(contents_str, -1)

    var res int
    var cond int = 1
    var cls_idx int
    for i, index := range mul_indexes {
        numbers := strings.Split(string(contents[index[0]+4:index[1]-1]), ",")
        x, _ := strconv.Atoi(numbers[0])
        y, _ := strconv.Atoi(numbers[1])

        if mul_indexes[i][0] > cond_indexes[0][0] {
            if cls_idx+1 < len(cond_indexes) {
                for cond_indexes[cls_idx+1][0] < mul_indexes[i][0] {
                    cls_idx++
                    if cls_idx+1 >= len(cond_indexes) {
                        break
                    }
                }
            }

            if cond_indexes[cls_idx][1] - cond_indexes[cls_idx][0] == 4 {
                cond = 1
            } else {
                cond = 0
            }
        } 

        res = res + (x*y*cond)
    }
    fmt.Println(res)
}

func main() {
    contents, _ := os.ReadFile("./input_2")
    sol1(contents)
    sol2(contents)
}
