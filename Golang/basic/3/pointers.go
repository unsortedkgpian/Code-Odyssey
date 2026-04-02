package main

import (
	"fmt"
)

func main(){
	fmt.Println("Hello Pointers!")

	age := 32

	agePointer := &age
	fmt.Println("Age pointer:", agePointer)
	fmt.Println("Age pointer:", *agePointer)
	fmt.Println("Age:", age)


	adultYears := getAdultYears(age)
	getAdultYearsPointer(agePointer)
	fmt.Println(adultYears)
	fmt.Println(age)
}

func getAdultYearsPointer(age *int){
	*age-=8
}

func getAdultYears(age int) int {
	return age -18
}