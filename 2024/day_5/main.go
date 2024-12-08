package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func sol1(rules string, updates string) {
    pages_map := make(map[string][]string)
    for _, rule := range strings.Split(rules, "\n") {
	pages := strings.Split(rule, "|")
	r := pages_map[pages[0]]
	pages_map[pages[0]] = append(r, pages[1])
    }

    var valid_updates []string
    for _, update := range strings.Split(updates, "\n") {
	if len(update) <= 0 {
	    continue
	}
	seen := make(map[string]bool)
	flag := true
	for _, page := range strings.Split(update, ",") {
	    for _, after := range pages_map[page] {
		if seen[after] {
		    flag = false
		}
	    }

	    seen[page] = true
	}
	if flag {
	    valid_updates = append(valid_updates, update)
	}
    }

    var res int
    for _, valid_update := range valid_updates {
	pages := strings.Split(valid_update, ",")
	middle, _ := strconv.Atoi(pages[(len(pages)/2)])
	res = res + int(middle)
    }
    fmt.Println(res)
}

func sol2(rules string, updates string) {
    pages_map := make(map[string][]string)
    for _, rule := range strings.Split(rules, "\n") {
	pages := strings.Split(rule, "|")
	r := pages_map[pages[0]]
	pages_map[pages[0]] = append(r, pages[1])
    }

    var fixed_updates [][]string
    for _, update := range strings.Split(updates, "\n") {
	if len(update) <= 0 {
	    continue
	}

	fixed := false
	seen := make(map[string]int)
	pages := strings.Split(update, ",")
	var idx int
	for idx < len(pages) {
	    for _, after := range pages_map[pages[idx]]{
		if seen[after] != 0 {
		    fixed = true
		    tmp_idx := seen[after]-1
		    tmp := pages[tmp_idx]
		    pages = slices.Delete(pages, tmp_idx, tmp_idx+1)
		    pages = slices.Insert(pages, idx, tmp)
		    seen[after] = 0
		    idx = idx - 1
		}
	    }
	    seen[pages[idx]] = idx+1
	    idx = idx + 1
	}
	if fixed {
	    fixed_updates = append(fixed_updates, pages)
	}
    }

    var res int
    for _, fixed_update := range fixed_updates {
	middle, _ := strconv.Atoi(fixed_update[(len(fixed_update)/2)])
	res = res + int(middle)
    }
    fmt.Println(res)
}

 
func main() {
    contents, _ := os.ReadFile("./input_2")
    parts := strings.Split(string(contents), "\n\n")
    sol1(parts[0], parts[1])
    sol2(parts[0], parts[1])
}
