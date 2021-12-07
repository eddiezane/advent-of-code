package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	stepone()
}

type v interface {
	forward(int)
	up(int)
	down(int)
	calc() int
}

type sub struct {
	x int
	y int
}

func (s *sub) forward(i int) {
	s.x += i
}

func (s *sub) up(i int) {
	s.y -= i
}

func (s *sub) down(i int) {
	s.y += i
}

func (s *sub) calc() int {
	return s.x * s.y
}

var _ v = &sub{}

func stepone() {
	if len(os.Args) != 2 {
		log.Fatal("usage: day-2 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)

	mySub := sub{}

	for s.Scan() {
		l := s.Text()

		split := strings.Split(l, " ")
		if len(split) != 2 {
			log.Fatal("something wrong parsing:", split)
		}
		direction, valueS := split[0], split[1]

		value, err := strconv.Atoi(valueS)
		if err != nil {
			log.Fatal(err)
		}

		switch direction {
		case "forward":
			mySub.forward(value)
		case "up":
			mySub.up(value)
		case "down":
			mySub.down(value)
		}
	}

	log.Println(mySub.calc())
}
