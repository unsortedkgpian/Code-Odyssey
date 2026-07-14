package main 

import (
	"fmt"
)

func main(){
	numbers := []int{1,2,32,2}
	dNumbers := transformNumbers(&numbers, triple)

	fmt.Println(numbers)
	fmt.Println(dNumbers)
}


func transformNumbers(numbers *[]int, transform func(int ) int ) []int{
	dNumbers := []int{}

	for _, val := range *numbers{
		dNumbers = append(dNumbers, transform(val))
	}

	return dNumbers
}


func double(number int) int{
	return number*2
}

func triple (number int) int{
	return number*3
}