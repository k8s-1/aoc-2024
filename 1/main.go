package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
	"bufio"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	var a []int
	var b []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		// Read line as string.
		line := scanner.Text()
		columns := strings.Fields(line)

		valA, _ := strconv.Atoi(columns[0])
		valB, _ := strconv.Atoi(columns[1])

		a = append(a, valA)
		b = append(b, valB)
	}

	sort.Ints(a)
	sort.Ints(b)

	sum := 0
	for i := range a {
		sum += int(math.Abs(float64(a[i] - b[i])))
	}

	fmt.Println(sum)
}
