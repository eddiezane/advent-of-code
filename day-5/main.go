package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type point struct {
	x    int
	y    int
	line *line
}

type line struct {
	start *point
	end   *point
}

type grid [][][]*point

func (g grid) addLine(l *line) {
	mx := int(math.Abs(float64(l.start.x) - float64(l.end.x)))
	my := int(math.Abs(float64(l.start.y) - float64(l.end.y)))

	// TODO: Pull this out
	if mx == 0 {
		// vertical
		ly := int(math.Min(float64(l.start.y), float64(l.end.y)))
		lj := int(math.Max(float64(l.start.y), float64(l.end.y)))
		for i := ly; i <= lj; i++ {
			p := &point{l.start.x, i, l.start.line}
			g[p.y][p.x] = append(g[p.y][p.x], p)
		}
	} else if my == 0 {
		// horizontal
		lx := int(math.Min(float64(l.start.x), float64(l.end.x)))
		ly := int(math.Max(float64(l.start.x), float64(l.end.x)))
		for i := lx; i <= ly; i++ {
			p := &point{i, l.start.y, l.start.line}
			g[p.y][p.x] = append(g[p.y][p.x], p)
		}
	}
}

func (g grid) print() {
	count := 0
	for _, r := range g {
		for _, c := range r {
			if len(c) == 0 {
				fmt.Print(".")
			} else {
				fmt.Print(len(c))
				if len(c) > 1 {
					count++
				}
			}
		}
		fmt.Print("\n")
	}
	fmt.Print("\n")

	fmt.Println(count)
}

func main() {
	if len(os.Args) != 2 {
		log.Fatal("usage: day-5 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)

	input := make([]string, 0)
	for s.Scan() {
		input = append(input, s.Text())
	}

	maxX := 0
	maxY := 0
	lines := make([]*line, 0)
	for _, l := range input {
		n := newLine(l)
		if n.isDiagonal() {
			continue
		}
		lines = append(lines, n)
		maxX = maxInt(n.start.x, n.end.x, maxX)
		maxY = maxInt(n.start.y, n.end.y, maxY)
	}
	// make grid
	g := newGrid(maxX+1, maxY+1)

	for _, l := range lines {
		g.addLine(l)
	}
	g.print()
}

func newGrid(x, y int) grid {
	g := make(grid, y)
	for i := 0; i < y; i++ {
		g[i] = make([][]*point, x)
	}
	return g
}

func newLine(s string) *line {
	l := &line{}
	a := strings.Split(s, " ")
	l.start = newPoint(a[0], l)
	l.end = newPoint(a[2], l)
	return l
}

func (l *line) isDiagonal() bool {
	return !((l.start.x == l.end.x) || (l.start.y == l.end.y))
}

func (l *line) points() []*point {
	ps := make([]*point, 0)

	// y = mx + b
	// b = y - mx
	mx := l.end.x - l.start.x
	my := l.end.y - l.start.y
	m := float64(my) / float64(mx)
	b := float64(l.start.y) - m*float64(l.start.x)

	log.Println(l.start, l.end, mx, my, m, b)

	return ps
}

func newPoint(s string, l *line) *point {
	a := strings.Split(s, ",")
	x, err := strconv.Atoi(a[0])
	if err != nil {
		log.Fatal(err)
	}
	y, err := strconv.Atoi(a[1])
	if err != nil {
		log.Fatal(err)
	}
	return &point{x, y, l}
}

func maxInt(ints ...int) int {
	max := 0
	for _, i := range ints {
		if i > max {
			max = i
		}
	}
	return max
}
