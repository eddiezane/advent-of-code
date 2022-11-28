package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

type flowGrid [][]int

type point struct {
	x int
	y int
}

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

	var grid flowGrid
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

	lowPoints := grid.lowPoints()

	log.Println("low points are: ", lowPoints)

	sum := 0
	for _, lp := range lowPoints {
		sum = sum + grid[lp.y][lp.x] + 1
	}

	log.Println("sum: ", sum)

	basins := grid.basins()

	sort.Slice(basins, func(i, j int) bool {
		return len(basins[i]) > len(basins[j])
	})

	log.Println("basin score: ", len(basins[0])*len(basins[1])*len(basins[2]))
}

func (grid flowGrid) findAdjacent(p point) []point {
	adjacentPoints := []point{
		{1, 0},
		{-1, 0},
		{0, 1},
		{0, -1},
	}

	var result []point

	for _, ap := range adjacentPoints {
		xx := p.x + ap.x
		yy := p.y + ap.y

		if xx >= 0 && xx < len(grid[0]) &&
			yy >= 0 && yy < len(grid) {
			result = append(result, point{xx, yy})
		}
	}

	return result
}

func (grid flowGrid) lowPoints() []point {
	var lowPoints []point
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			base := grid[y][x]

			lp := true

			ap := grid.findAdjacent(point{x, y})

			for _, p := range ap {
				xx := p.x
				yy := p.y
				if base >= grid[yy][xx] {
					lp = false
				}
			}

			if lp {
				lowPoints = append(lowPoints, point{x, y})
			}
		}
	}
	return lowPoints
}

func (grid flowGrid) basins() [][]point {
	var res [][]point

	lowPoints := grid.lowPoints()

	for _, lp := range lowPoints {
		visited := make(map[point]bool)
		var queue []point
		var basin []point

		visited[lp] = true
		queue = append(queue, lp)
		basin = append(basin, lp)

		for len(queue) != 0 {
			v := queue[0]
			queue = queue[1:]

			ap := grid.findAdjacent(v)

			greater := grid.pointsGreater(ap, v)

			for _, p := range greater {
				if !visited[p] {
					visited[p] = true
					basin = append(basin, p)
					queue = append(queue, p)
				}
			}
		}
		res = append(res, basin)
	}

	return res
}

func (grid flowGrid) pointsGreater(points []point, p point) []point {
	var res []point

	v := grid[p.y][p.x]

	for _, i := range points {
		vx := grid[i.y][i.x]
		if vx > v && vx != 9 {
			res = append(res, i)
		}
	}
	return res
}
