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
	mx := l.start.x - l.end.x
	my := l.start.y - l.end.y

	yMin := int(math.Min(float64(l.start.y), float64(l.end.y)))
	yMax := int(math.Max(float64(l.start.y), float64(l.end.y)))

	xMin := int(math.Min(float64(l.start.x), float64(l.end.x)))
	xMax := int(math.Max(float64(l.start.x), float64(l.end.x)))

	// TODO: Pull this out into delta func
	if mx == 0 {
		// vertical
		for i := yMin; i <= yMax; i++ {
			p := &point{l.start.x, i, l.start.line}
			g[p.y][p.x] = append(g[p.y][p.x], p)
		}
	} else if my == 0 {
		// horizontal
		for i := xMin; i <= xMax; i++ {
			p := &point{i, l.start.y, l.start.line}
			g[p.y][p.x] = append(g[p.y][p.x], p)
		}
	} else {
		// 45 diagonal
		m := my / mx

		if m < 0 {
			// line goes /
			for x, y := xMin, yMax; x <= xMax && y >= yMin; {
				p := &point{x, y, l.start.line}
				g[p.y][p.x] = append(g[p.y][p.x], p)

				x++
				y--
			}
		} else {
			// line goes \
			for x, y := xMin, yMin; x <= xMax && y <= yMax; {
				p := &point{x, y, l.start.line}
				g[p.y][p.x] = append(g[p.y][p.x], p)

				x++
				y++
			}
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
	log.SetFlags(log.Llongfile)

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
