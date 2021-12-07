package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func main() {
	steptwo()
}

func stepone() {
	if len(os.Args) != 2 {
		log.Fatal("usage: day-1 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)

	var prev *int
	count := 0
	for s.Scan() {
		l := s.Text()

		i, err := strconv.Atoi(l)
		if err != nil {
			log.Fatal(err)
		}

		if prev != nil && i > *prev {
			count++
		}
		prev = &i
	}
	log.Println("count: ", count)
}

type window []int

func (w window) sum() int {
	return w[0] + w[1] + w[2]
}

func (w window) push(i int) window {
	w[0], w[1] = w[1], w[2]
	w[2] = i
	return w
}

func steptwo() {

	if len(os.Args) != 2 {
		log.Fatal("usage: day-1 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)

	w := make(window, 3)
	count := 0

	for i := 0; i < 3; i++ {
		s.Scan()
		l := s.Text()
		j, err := strconv.Atoi(l)
		if err != nil {
			log.Fatal(err)
		}
		w.push(j)
	}

	prev := w.sum()
	log.Println(w, prev)

	for s.Scan() {
		l := s.Text()

		i, err := strconv.Atoi(l)
		if err != nil {
			log.Fatal(err)
		}

		w.push(i)

		sum := w.sum()
		log.Println(w, sum)
		if w.sum() > prev {
			count++
		}
		prev = sum
	}
	log.Println("count: ", count)
}
