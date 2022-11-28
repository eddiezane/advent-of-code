package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

type line struct {
	signal [][]string
	output []string
}

type sim struct {
	lines []*line
}

func main() {
	log.SetFlags(log.Llongfile)

	if len(os.Args) != 2 {
		log.Fatal("usage: day-8 input.txt")
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
	l.signal = make([][]string, 0)
	l.output = make([]string, 0)

	parts := strings.Split(s, "|")

	signalLine := parts[0]
	for _, s := range strings.Split(strings.TrimSpace(signalLine), " ") {
		l.signal = append(l.signal, strings.Split(s, ""))
	}

	outputLine := parts[1]
	for _, s := range strings.Split(strings.TrimSpace(outputLine), " ") {
		l.output = append(l.output, s)
	}
	return l
}

func (l *line) solve() int {
	sm := newSegMap()

	// Find #1 which has 2 segs
	// This gives s2 s5
	for _, digit := range l.signal {
		if len(digit) == 2 {
			sm[2] = append(sm[2], digit...)
			sm[5] = append(sm[5], digit...)
		}
	}

	// Find #7 which has 3 segs
	// This gives us s0 if we diff with s2
	for _, digit := range l.signal {
		if len(digit) == 3 {
			sm[0] = append(sm[0], diff(digit, sm[2])...)
		}
	}

	// Find #4 which has 4 segs
	// This gives us s1 s3
	for _, digit := range l.signal {
		if len(digit) == 4 {
			sm[1] = append(sm[1], diff(digit, sm[2])...)
			sm[3] = append(sm[3], diff(digit, sm[2])...)
		}
	}

	// Find #6 which will be missing s2 or s5
	// It has len 6. #6 will be the only len(6) without both s2 and s5
	// This confirms s2 and s5
	sixLength := make([][]string, 0)
	six := make([]string, 0)
	for _, digit := range l.signal {
		if len(digit) == 6 {
			if !(has(digit, sm[2][0]) && has(digit, sm[2][1])) {
				six = append(six, digit...)
			} else {
				sixLength = append(sixLength, digit)
			}
		}
	}
	sm[2] = intersect(diff(sixLength[0], six), sm[5])
	sm[5] = diff(sm[5], sm[2])

	// Get difference between #0 and #9
	// This will be s3 s4
	threeFour := diff(sixLength[0], sixLength[1])
	sm[3] = intersect(threeFour, sm[3])
	sm[4] = diff(sm[3], threeFour)
	sm[1] = diff(sm[3], sm[1])

	// Find s6 from all others
	segSix := []string{sm[0][0], sm[1][0], sm[2][0], sm[3][0], sm[4][0], sm[5][0]}
	sm[6] = diff(segSix, []string{"a", "b", "c", "d", "e", "f", "g"})

	zero := []string{sm[0][0], sm[1][0], sm[2][0], sm[4][0], sm[5][0], sm[6][0]}
	one := []string{sm[2][0], sm[5][0]}
	two := []string{sm[0][0], sm[2][0], sm[3][0], sm[4][0], sm[6][0]}
	three := []string{sm[0][0], sm[2][0], sm[3][0], sm[5][0], sm[6][0]}
	four := []string{sm[1][0], sm[2][0], sm[3][0], sm[5][0]}
	five := []string{sm[0][0], sm[1][0], sm[3][0], sm[5][0], sm[6][0]}
	seven := []string{sm[0][0], sm[2][0], sm[5][0]}
	eight := []string{sm[0][0], sm[1][0], sm[2][0], sm[3][0], sm[4][0], sm[5][0], sm[6][0]}
	nine := []string{sm[0][0], sm[1][0], sm[2][0], sm[3][0], sm[5][0], sm[6][0]}

	numbers := [][]string{zero, one, two, three, four, five, six, seven, eight, nine}

	key := make(map[string]string)
	for i, number := range numbers {
		sort.Strings(number)
		key[strings.Join(number, "")] = strconv.Itoa(i)
	}

	sb := strings.Builder{}
	for _, s := range l.output {
		digit := strings.Split(s, "")
		sort.Strings(digit)
		sb.WriteString(key[strings.Join(digit, "")])
	}
	atoi, _ := strconv.Atoi(sb.String())
	return atoi
}

// func newDigitMap() map[int][]int {
// }

func newCharMap() map[int]int {
	charMap := make(map[int]int)

	// segments to number
	charMap[2] = 1
	charMap[4] = 4
	charMap[3] = 7
	charMap[7] = 8

	return charMap
}

func newSegMap() map[int][]string {
	sm := make(map[int][]string)

	for i := 0; i < 7; i++ {
		sm[i] = make([]string, 0)
	}

	return sm
}

func (s *sim) run() {
	sum := 0
	for _, l := range s.lines {
		sum = sum + l.solve()
	}
	log.Println(sum)
}

func diff(a, b []string) []string {
	m := make(map[string]bool)
	for _, s := range b {
		m[s] = true
	}
	d := make([]string, 0)
	for _, s := range a {
		if v := m[s]; !v {
			d = append(d, s)
		}
	}
	m = make(map[string]bool)
	for _, s := range a {
		m[s] = true
	}
	for _, s := range b {
		if v := m[s]; !v {
			d = append(d, s)
		}
	}
	return d
}

func intersect(a, b []string) []string {
	m := make(map[string]bool)
	for _, s := range a {
		m[s] = true
	}
	inter := make([]string, 0)
	for _, s := range b {
		if v := m[s]; v {
			inter = append(inter, s)
		}
	}
	return inter
}

func has(list []string, letter string) bool {
	for _, s := range list {
		if s == letter {
			return true
		}
	}

	return false
}
