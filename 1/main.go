package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	a := []int{3, 4, 2, 1, 3, 3}
	b := []int{4, 3, 5, 3, 9, 3}

	sort.Ints(a) // Sort array a
	sort.Ints(b) // Sort array a

	sum := 0
	for i := range a {
		sum += int(math.Abs(float64(a[i] - b[i])))
	}

	fmt.Println(sum)
}
