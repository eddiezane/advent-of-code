package main

import (
	"fmt"
	"github.com/eddiezane/advent-of-code-2021/util"
	"strconv"
	"strings"
)

type grid struct {
	data [][]*octopus
}

type octopus struct {
	energy  int
	flashed bool
	row     int
	col     int
	grid    *grid
}

func newOctopus(row, col, energy int, g *grid) *octopus {
	return &octopus{
		energy: energy,
		row:    row,
		col:    col,
		grid:   g,
	}
}

func newGrid(input []string) *grid {
	g := &grid{}
	g.data = make([][]*octopus, len(input))
	for row := 0; row < len(input); row++ {
		c := make([]*octopus, len(input[row]))
		r := input[row]
		split := strings.Split(r, "")
		for col, s := range split {
			i, _ := strconv.Atoi(s)
			o := newOctopus(row, col, i, g)
			c[col] = o
		}
		g.data[row] = c
	}
	return g
}

func (o *octopus) String() string {
	return strconv.Itoa(o.energy)
}

func (o *octopus) inc() {
	o.energy++
}

func (o *octopus) zero() {
	if o.energy > 9 {
		o.energy = 0
	}
	o.flashed = false
}

func (o *octopus) flash() bool {
	if o.flashed || o.energy <= 9 {
		return false
	}

	o.flashed = true

	for _, o2 := range o.adjacent() {
		o2.inc()
	}

	return true
}

func (o *octopus) adjacent() []*octopus {
	var adj []*octopus

	want := []point{
		{-1, -1},
		{0, -1},
		{1, -1},
		{1, 0},
		{1, 1},
		{0, 1},
		{-1, 1},
		{-1, 0},
	}

	for _, p := range want {
		xx := p.x + o.col
		yy := p.y + o.row

		if yy >= 0 && yy < len(o.grid.data) &&
			xx >= 0 && xx < len(o.grid.data[yy]) {
			adj = append(adj, o.grid.data[yy][xx])
		}

	}

	return adj
}

type point struct {
	x int
	y int
}

func (g *grid) step() int {
	for _, row := range g.data {
		for _, col := range row {
			col.inc()
		}
	}

	flashes := 0
	run := true
	for run {
		flashed := false
		for _, row := range g.data {
			for _, col := range row {
				if col.flash() {
					flashes++
					flashed = true
				}
			}
		}
		if !flashed {
			run = false
		}
	}

	for _, row := range g.data {
		for _, col := range row {
			col.zero()
		}
	}

	return flashes
}

func (g *grid) String() string {
	sb := new(strings.Builder)
	for row := 0; row < len(g.data); row++ {
		for col := 0; col < len(g.data[row]); col++ {
			sb.WriteString(g.data[row][col].String())
		}
		sb.WriteString("\n")
	}
	return sb.String()
}

func main() {
	input := util.Slurp()
	g := newGrid(input)

	steps := 100
	flashes := 0
	for i := 0; i < steps; i++ {
		flashes = flashes + g.step()
	}

	fmt.Println(g)
	fmt.Println(flashes)
}
