package main

import (
	"fmt"
)

type Product struct {
	title string
	id int
	prices float64
}

func main(){
	hobbies := [3]string{"Coding", "Cricket", "gaming"}
	fmt.Println(hobbies)
	fmt.Println(hobbies[0])
	fmt.Println(hobbies[1:])

	hobbiesSlices := hobbies[:2]
	fmt.Println(hobbiesSlices)
	hobbiesSlices = hobbies[1:]
	fmt.Println(hobbiesSlices)

	goals := []string{"30lpa", "newtable", "newme"}
	fmt.Println(goals)

	combine := append(goals, hobbies[:]...)
	fmt.Println(combine)

	dynamic := []Product{Product{title:"HOA", id:2, prices:3.42}, Product{title:"Neo", id:32, prices:8.32}}
	fmt.Println(dynamic)

	dynamic = append(dynamic, Product{title:"Sheikh", id:74, prices:3.2})
	fmt.Println(dynamic)
}

// func main(){
// 	var productNames [4]string
// 	prices := [4]float64{32, 23.2, 53, 53.23}
// 	productNames = [4]string{"a", "bc", "def", "ghij"}

// 	fmt.Println(prices[2])
// 	fmt.Println(productNames)

// 	selectedProduct := productNames[1:3]
// 	fmt.Println(selectedProduct)
// 	first := productNames[1:]
// 	fmt.Println(len(first), cap(first))
// 	second := productNames[2:3]
// 	fmt.Println(len(second), cap(second))
// 	second = productNames[1:]
// 	fmt.Println(len(second), cap(second))


// 	dynamic := []float64{23, 3829.293}
// 	fmt.Println(dynamic, len(dynamic), cap(dynamic))
// 	dynamic = append(dynamic, 337.9)
// 	fmt.Println(dynamic, len(dynamic), cap(dynamic))
// 	dynamic = append(dynamic, 33.9)
// 	fmt.Println(dynamic, len(dynamic), cap(dynamic))
// 	dynamic = append(dynamic, .839)
// 	fmt.Println(dynamic, len(dynamic), cap(dynamic))


// }