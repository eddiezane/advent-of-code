package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

type space struct {
	number int
	marked bool
}

type board [][]*space

func (b board) mark(i int) {
	for _, x := range b {
		for _, y := range x {
			if i == y.number {
				y.marked = true
			}
		}
	}
}

func (b board) print() {
	for _, r := range b {
		for _, s := range r {
			log.Println(*s)
		}
	}
}

func (b board) isWinner() bool {
	// check rows
	for _, r := range b {
		// assume the row is a winner until a false is found in a space
		won := true
		for _, s := range r {
			if !s.marked {
				won = false
				break
			}
		}
		if won {
			return true
		}
	}

	// check cols
	colsWon := make([]bool, 5)
	for i := range colsWon {
		colsWon[i] = true
	}
	for _, r := range b {
		for i, s := range r {
			if !s.marked {
				colsWon[i] = false
			}
		}
	}
	for _, i := range colsWon {
		if i {
			return true
		}
	}

	return false
}

// score takes in the last number called
func (b board) score(n int) int {
	sum := 0

	for _, r := range b {
		for _, s := range r {
			if !s.marked {
				sum += s.number
			}
		}
	}
	return sum * n
}

func allBoardsWon(boards []board) bool {
	for _, b := range boards {
		if !b.isWinner() {
			return false
		}
	}
	return true
}

func main() {
	log.SetFlags(log.Lshortfile)

	if len(os.Args) != 2 {
		log.Fatal("usage: day-4 input.txt")
	}

	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)

	// read the first line of numbers called
	s.Scan()
	l := s.Text()

	numbers := make([]int, 0)
	for _, i := range strings.Split(l, ",") {
		n, err := strconv.Atoi(i)
		if err != nil {
			log.Fatal(err)
		}
		numbers = append(numbers, n)
	}

	// read a blank line and assume the next 5 are boards
	boards := make([]board, 0)
	for s.Scan() {
		nb := make(board, 0)
		for i := 0; i < 5; i++ {
			row := make([]*space, 0)
			s.Scan()
			l = s.Text()
			for _, y := range strings.Split(l, " ") {
				if y == "" {
					continue
				}
				n, err := strconv.Atoi(y)
				if err != nil {
					log.Fatal(err, y)
				}
				row = append(row, &space{number: n})
			}
			nb = append(nb, row)
		}
		boards = append(boards, nb)
	}

	for _, n := range numbers {
		for _, b := range boards {
			b.mark(n)
			if b.isWinner() {
				if allBoardsWon(boards) {
					log.Println(b.score(n))
					return
				}
			}
		}
	}
}
