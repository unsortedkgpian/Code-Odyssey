package main

import "fmt"

func main(){
	var nums [8]int // -> make all element 0
	nums[0] =32
	nums[2] =54
	nums[3] = 45
	nums[4] = 22
	nums[1] =98

	fmt.Println("the lenght of array is ",len(nums))
	for i :=range 8{
		fmt.Println(nums[i])
	}
	fmt.Println(nums)
	// -> wrong println(nums)

	var boolen [4]bool 
	fmt.Println(boolen)

	var stringa [4]string 
	fmt.Println(stringa)


	numso := [3]int{3,44,2}
	fmt.Println(numso)


	twodnums:=[3][2]int{{4,2},{8,0}}
	fmt.Println(twodnums);
}
