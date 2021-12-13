package main

import (
	"bufio"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type crab struct {
	start int
}

func newCrab(i int) *crab {
	return &crab{i}
}

func (c *crab) fuel(target int) int {
	return int(math.Abs(float64(c.start - target)))
}

type sim struct {
	crabs []*crab
	max   int
}

func newSim(input []int) *sim {
	s := &sim{}
	s.crabs = make([]*crab, len(input))
	for i, v := range input {
		if v > s.max {
			s.max = v
		}
		s.crabs[i] = newCrab(v)
	}
	return s
}

func (s *sim) run() {
	costs := make([]int, s.max)
	for i := 0; i < s.max; i++ {
		cost := s.calc(i)
		costs[i] = cost
		log.Printf("pos: %d cost: %d", i, cost)
	}
	lowest := 0
	index := 0
	for i, cost := range costs {
		if i == 0 || cost < lowest {
			lowest = cost
			index = i
		}
	}
	log.Printf("lowest: %d cost: %d", index, lowest)
}

func (s *sim) calc(target int) int {
	total := 0
	for _, c := range s.crabs {
		total = total + c.fuel(target)
	}
	return total
}

func main() {
	log.SetFlags(log.Llongfile)

	if len(os.Args) != 2 {
		log.Fatal("usage: day-7 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)

	input := make([]int, 0)
	sc.Scan()
	for _, n := range strings.Split(sc.Text(), ",") {
		i, _ := strconv.Atoi(n)
		input = append(input, i)
	}

	s := newSim(input)
	s.run()
}
