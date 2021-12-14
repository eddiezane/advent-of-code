package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

type line struct {
	signal []string
	output []string
}

type sim struct {
	lines []*line
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

	lines := make([]*line, 0)
	for sc.Scan() {
		l := sc.Text()
		lines = append(lines, newLine(l))
	}

	s := newSim(lines)

	s.run()
}

func newSim(lines []*line) *sim {
	return &sim{
		lines: lines,
	}
}

func newLine(s string) *line {
	l := &line{}
	l.signal = make([]string, 0)
	l.output = make([]string, 0)

	parts := strings.Split(s, "|")

	signalLine := parts[0]
	for _, s := range strings.Split(strings.TrimSpace(signalLine), " ") {
		l.signal = append(l.signal, s)
	}

	outputLine := parts[1]
	for _, s := range strings.Split(strings.TrimSpace(outputLine), " ") {
		l.output = append(l.output, s)
	}
	return l
}

func newCharMap() map[int]int {
	charMap := make(map[int]int, 0)

	// segments to number
	charMap[2] = 1
	charMap[4] = 4
	charMap[3] = 7
	charMap[7] = 8

	return charMap
}

func (s *sim) run() {
	charMap := newCharMap()
	result := make(map[int]int, 0)

	for _, l := range s.lines {
		for _, o := range l.output {
			result[charMap[len(o)]]++
		}
	}
	log.Println(result)

	sum := 0
	for k, v := range result {
		if k == 0 {
			continue
		}

		sum = sum + v
	}
	log.Println(sum)
}
