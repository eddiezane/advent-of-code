package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	log.SetFlags(log.Llongfile)

	if len(os.Args) != 2 {
		log.Fatal("usage: day-9 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)

	var grid [][]int
	for sc.Scan() {
		text := sc.Text()
		split := strings.Split(text, "")
		var row []int
		for _, s := range split {
			atoi, _ := strconv.Atoi(s)
			row = append(row, atoi)
		}
		grid = append(grid, row)
	}

	var lowPoints []int
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			point := grid[y][x]
			top, bottom, left, right := 9, 9, 9, 9

			// top
			if y != 0 {
				top = grid[y-1][x]
			}
			// bottom
			if y != len(grid)-1 {
				bottom = grid[y+1][x]
			}
			// left
			if x != 0 {
				left = grid[y][x-1]
			}
			// right
			if x != len(grid[0])-1 {
				right = grid[y][x+1]
			}

			if point < top && point < bottom && point < left && point < right {
				lowPoints = append(lowPoints, grid[y][x]+1)
			}
		}
	}

	log.Println(lowPoints)

	sum := 0
	for _, i := range lowPoints {
		sum = sum + i
	}

	log.Println(sum)
}
