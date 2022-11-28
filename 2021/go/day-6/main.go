package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

type sim struct {
	bucket []int
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
}

func newSim(input []int) *sim {
	s := &sim{}
	s.bucket = make([]int, 9)
	for _, i := range input {
		s.bucket[i]++
	}
	return s
}

func (s *sim) run(days int) {
	log.Printf("Initial state: %v", s.bucket)
	for i := 0; i < days; i++ {
		newFish := s.bucket[0]
		for j := 0; j < 8; j++ {
			s.bucket[j] = s.bucket[j+1]
		}
		s.bucket[6] = s.bucket[6] + newFish
		s.bucket[8] = newFish
		log.Printf("Day %d: %v %d", i+1, s.bucket, s.sum())
	}
}

func (s *sim) sum() int {
	count := 0
	for _, i := range s.bucket {
		count = count + i
	}
	return count
}
