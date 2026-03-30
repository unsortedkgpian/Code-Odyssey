package main

import (
	"fmt"
)

func main(){

	var accountBalance float64 = 1000
	// fmt.Printf("Hello world")
	fmt.Println("Welcome to Go Bank! Hola Amigo")
	fmt.Println("What do you want to do?")
	fmt.Println("1. Check balance")
	fmt.Println("2. Deposit money")
	fmt.Println("3. Withdraw money")
	fmt.Println("4. Exit")

	var choice int
	fmt.Print("Your Input: ")
	fmt.Scan(&choice)
	fmt.Println("Your Choice: ", choice)



	if choice == 1{
		// fmt.Println("Hi how are you")
		fmt.Printf("Your balance is %v\n", accountBalance)
	}else if choice == 2{
		fmt.Print("Your deposit: ")
		var depositAmount float64
		fmt.Scan(&depositAmount)
		accountBalance += depositAmount
		fmt.Println("Balance update! Your new balance is:", accountBalance)
	}else if choice == 3{
		fmt.Print("How much you want to withdraw:")
		var withdrawAmount float64
		fmt.Scan(&withdrawAmount)
		if withdrawAmount > accountBalance {
			fmt.Println("Insufficent Balance")
		}else {
			accountBalance -=withdrawAmount
			fmt.Println("Your remaning balance is :", accountBalance)
		}
	}else{
		
	}



}