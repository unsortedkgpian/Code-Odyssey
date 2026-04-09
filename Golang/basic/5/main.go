package main

import (
	"fmt"
	"errors"
)

// type str string

// func (text str) log() {
// 	fmt.Println(text)
// }

func main(){
	// var name str
	// name = "Raj"
	// name.log()
	// fmt.Printf("%t",name)

	

}

func getUserInput(promt string) (string, error){
	fmt.Println(promt)
	var value string 
	fmt.Scan(&value)


	if value == ""{
		fmt.Println("Value can't be empty")
		return "" , errors.New("Value can't be empty")
	}
	return value, nil
}