package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

type fish struct {
	timer int
}

func (f *fish) tick(s *sim) {
	f.timer--
	if f.timer < 0 {
		f.timer = 6
		nf := newFish(8)
		s.addFish(nf)
	}
}

type sim struct {
	today   int
	fish    []*fish
	newFish []*fish
}

func (s *sim) addFish(f *fish) {
	s.newFish = append(s.newFish, f)
}

func (s *sim) run(days int) {
	log.Printf("Initial state: %s", s)
	for i := 0; i < days; i++ {
		for _, f := range s.fish {
			f.tick(s)
		}
		s.fish = append(s.fish, s.newFish...)
		s.newFish = make([]*fish, 0)
		log.Printf("After %d days: %s", i+1, s)
	}
}

func (s *sim) String() string {
	out := make([]string, 0)
	for _, f := range s.fish {
		itoa := strconv.Itoa(f.timer)
		out = append(out, itoa)
	}
	return strings.Join(out, ",")
}

func main() {
	log.SetFlags(log.Llongfile)

	if len(os.Args) != 3 {
		log.Fatal("usage: day-6 input.txt daysToSim")
	}

	daysToSim, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal(err)
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)

	input := make([]int, 0)
	for sc.Scan() {
		l := sc.Text()
		split := strings.Split(l, ",")
		for _, i := range split {
			atoi, err := strconv.Atoi(i)
			if err != nil {
				log.Fatal(err)
			}
			input = append(input, atoi)
		}
	}

	s := newSim(input)
	s.run(daysToSim)
	log.Printf("Total fish: %d", len(s.fish))
}

func newSim(input []int) *sim {
	s := &sim{}
	for _, i := range input {
		f := newFish(i)
		s.fish = append(s.fish, f)
	}
	return s
}

func newFish(i int) *fish {
	return &fish{i}
}
