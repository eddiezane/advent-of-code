package main

import (
	"bufio"
	"log"
	"os"
)

func main() {
	stepone()
}

func stepone() {
	if len(os.Args) != 2 {
		log.Fatal("usage: day-3 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)

	for s.Scan() {
		l := s.Text()
	}
}
