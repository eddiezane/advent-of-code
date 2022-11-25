package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strings"
)

type stack struct {
	data []string
}

func newStack() *stack {
	s := &stack{}
	s.data = make([]string, 0)
	return s
}

func (s *stack) push(item string) {
	s.data = append(s.data, item)
}

func (s *stack) pop() string {
	item := s.data[len(s.data)-1]
	s.data = s.data[:len(s.data)-1]
	return item
}

func (s *stack) empty() bool {
	return !(len(s.data) > 0)
}

func (s *stack) peek() string {
	return s.data[len(s.data)-1]
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

	openChars := map[string]int{
		"(": 3,
		"[": 57,
		"{": 1197,
		"<": 25137,
	}
	closeChars := map[string]int{
		")": 3,
		"]": 57,
		"}": 1197,
		">": 25137,
	}
	matchChars := map[string]string{
		"(": ")",
		"[": "]",
		"{": "}",
		"<": ">",
	}

	lineCounter := 0
	score := 0

	completionScores := make([]int, 0)

loop:
	for sc.Scan() {
		lineCounter++

		openStack := newStack()

		line := sc.Text()
		split := strings.Split(line, "")
		for _, char := range split {
			if _, ok := openChars[char]; ok {
				openStack.push(char)
			} else if points, ok := closeChars[char]; ok {
				o := openStack.peek()
				if c := matchChars[o]; c == char {
					openStack.pop()
				} else {
					log.Printf("line %d corrupted. expected %s found %s", lineCounter, c, char)
					log.Println(line)
					score = score + points
					goto loop
				}
			} else {
				log.Fatal("invalid char: ", char)
			}
		}

		if openStack.empty() {
			continue
		}

		log.Printf("line %d stack %v", lineCounter, openStack.data)
		autoScore := 0
		for !openStack.empty() {
			o := openStack.pop()
			c := matchChars[o]

			switch c {
			case ")":
				autoScore = autoScore*5 + 1
			case "]":
				autoScore = autoScore*5 + 2
			case "}":
				autoScore = autoScore*5 + 3
			case ">":
				autoScore = autoScore*5 + 4
			}
		}
		completionScores = append(completionScores, autoScore)
		log.Println("auto score:", autoScore)
	}

	sort.Ints(completionScores)

	log.Println("score: ", score)

	log.Println("middle completion score", completionScores[len(completionScores)/2])
}
