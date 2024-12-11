package main

import "fmt"

func main() {
	var l1 = [6]int{3, 4, 2, 1, 3, 3}
	var l2 = [6]int{4, 3, 5, 3, 9, 3}

	for _, v := range l1 {
		fmt.Println(v)
		for _, v := range l2 {
			fmt.Println(v)
		}
	}
}
