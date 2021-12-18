package util

import (
	"bufio"
	"log"
	"os"
)

func init() {
	log.SetFlags(log.Llongfile)
}

func Slurp() []string {
	if len(os.Args) != 2 {
		log.Fatal("usage: day-# input.txt")
	}

	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	sc := bufio.NewScanner(file)

	var lines []string
	for sc.Scan() {
		text := sc.Text()
		lines = append(lines, text)
	}

	return lines
}
