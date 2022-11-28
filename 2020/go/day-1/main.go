package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	s := strings.Split(input, "\n")

	numbers := make([]int, 0)
	for _, item := range s {
		i, err := strconv.Atoi(item)
		if err == nil {
			numbers = append(numbers, i)
		}
	}

	for i, n := range numbers {
		for j := i + 1; j < len(numbers); j++ {
			for k := j + 1; k < len(numbers); k++ {
				nn := numbers[j]
				nnn := numbers[k]
				fmt.Printf("n: %d nn: %d nnn: %d\n", n, nn, nnn)
				if n+nn+nnn == 2020 {
					fmt.Printf("%d + %d + %d == 2020; %d * %d * %d == %d\n", n, nn, nnn, n, nn, nnn, n*nn*nnn)
					goto done
				}
			}
		}
	}

done:
}
