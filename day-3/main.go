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

	input := make([]string, 0)
	for s.Scan() {
		input = append(input, s.Text())
	}

	bitLength := len(input[0])
	sums := make([]int, bitLength)

	for _, v := range input {
		bits := strings.Split(v, "")
		for ix, iv := range bits {
			bit, err := strconv.Atoi(iv)
			if err != nil {
				log.Fatal("error parsing bit:", err)
			}
			sums[ix] += bit
		}
	}

	gamma := ""
	eps := ""
	for _, v := range sums {
		if float64(v) > (float64(len(input)) / 2) {
			gamma = gamma + "1"
			eps = eps + "0"
		} else {
			gamma = gamma + "0"
			eps = eps + "1"
		}
	}

	gi, err := strconv.ParseUint(gamma, 2, len(sums))
	if err != nil {
		log.Fatal("error parsing gamma:", err)
	}

	ei, err := strconv.ParseUint(eps, 2, len(sums))
	if err != nil {
		log.Fatal("error parsing epsilon:", err)
	}

	log.Println(gi * ei)
}
