package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
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

	sub := input
	for i := 0; i < bitLength; i++ {
		if len(sub) == 1 {
			log.Println("sub is 1. breaking", sub)
			break
		}
		sum := 0
		for _, v := range sub {
			bit, err := strconv.Atoi(string(v[i]))
			if err != nil {
				log.Fatal("error parsing bit", err)
			}
			sum += bit
		}
		majority := 0
		if float64(sum) >= (float64(len(sub)) / 2) {
			majority = 1
		}
		newSub := make([]string, 0)
		for _, v := range sub {
			bit, err := strconv.Atoi(string(v[i]))
			if err != nil {
				log.Fatal("error parsing bit", err)
			}
			if bit == majority {
				newSub = append(newSub, v)
			}
		}
		sub = newSub
	}
	ox := sub[0]

	// lazy copy paste
	sub = input
	for i := 0; i < bitLength; i++ {
		if len(sub) == 1 {
			log.Println("sub is 1. breaking", sub)
			break
		}
		sum := 0
		for _, v := range sub {
			bit, err := strconv.Atoi(string(v[i]))
			if err != nil {
				log.Fatal("error parsing bit", err)
			}
			sum += bit
		}
		pass := 0
		if float64(sum) < (float64(len(sub)) / 2) {
			pass = 1
		}
		newSub := make([]string, 0)
		for _, v := range sub {
			bit, err := strconv.Atoi(string(v[i]))
			if err != nil {
				log.Fatal("error parsing bit", err)
			}
			if bit == pass {
				newSub = append(newSub, v)
			}
		}
		sub = newSub
	}
	co2 := sub[0]

	log.Println(ox, co2)

	oxi, err := strconv.ParseUint(ox, 2, bitLength)
	if err != nil {
		log.Fatal(err)
	}
	co2i, err := strconv.ParseUint(co2, 2, bitLength)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(oxi, co2i)
	log.Println(oxi * co2i)
}
