package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type block struct {
    id int
    start int
    size int

}

func sol1(contents *[]byte) {
    full, empty := parseInput(contents)

    full_block := pop(&full)
    empty_block := popFront(&empty)
    for true {
	if empty_block.size == full_block.size {
	    full_block.start = empty_block.start
	    pushFront(&full, full_block)

	    empty_block.size = 0
	    full_block.size = 0
	} else if empty_block.size > full_block.size {
	    full_block.start = empty_block.start
	    pushFront(&full, full_block)

	    empty_block.start += full_block.size
	    empty_block.size -= full_block.size
	    full_block.size -= full_block.size
	} else if empty_block.size < full_block.size {
	    empty_block.id = full_block.id
	    pushFront(&full, empty_block)

	    full_block.size -= empty_block.size
	    empty_block.size -= empty_block.size
	}

	if full_block.size == 0 { full_block = pop(&full) }
	if empty_block.size == 0 {
	    if len(empty) == 0 {
		pushFront(&full, full_block)
		break
	    }
	    empty_block = popFront(&empty)
	}
	if full_block.start < empty_block.start {
	    pushFront(&full, full_block)
	    break
	}
    }
    var res int
    for _, full_block := range full {
	for offset := range full_block.size {
	    res += (full_block.start+offset)*full_block.id
	}
    }
    fmt.Println(res)
}

func parseInput(contents *[]byte) ([]block, []block) {
    var full []block
    var empty []block

    pos := 0
    id := 0
    for idx, b := range strings.TrimSpace(string(*contents)) {
	b_val, _ := strconv.Atoi(string(b))
	if b_val == 0 {continue}
	switch idx%2 {
	case 0:
	    full = append(full, block{id: id, start: pos, size: b_val})
	    id++
	case 1:
	    empty = append(empty, block{id: -1, start: pos, size: b_val})
	}
	pos += b_val
    }
    return full, empty
}

func sol2(contents *[]byte) {
    full, empty := parseInput(contents)

    for i:=len(full)-1; i>=0; i-- {
	for j:=0; j<len(empty); j++ {
	    if full[i].size <= empty[j].size {
		if empty[j].start < full[i].start {
		    full[i].start = empty[j].start

		    empty[j].start += full[i].size
		    empty[j].size -= full[i].size
		}

	    }
	}
    }

    var res int
    for _, full_block := range full {
	for offset := range full_block.size {
	    res += (full_block.start+offset)*full_block.id
	}
    }
    fmt.Println(res)
}

func pop(blocks *[]block) block {
    new_blocks := *blocks
    b := new_blocks[len(new_blocks)-1]
    new_blocks = new_blocks[:len(new_blocks)-1]
    *blocks = new_blocks
    return b
}

func popFront(blocks *[]block) block {
    new_blocks := *blocks
    b := new_blocks[0]
    new_blocks = new_blocks[1:]
    *blocks = new_blocks
    return b
}

func pushFront(blocks *[]block, b block) {
    new_blocks := *blocks
    new_blocks = append([]block{b}, new_blocks...)
    *blocks = new_blocks
}

func main() {
    contents, _ := os.ReadFile("./input_2")
    sol1(&contents)
    sol2(&contents)

}
