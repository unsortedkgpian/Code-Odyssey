package main

import (
	"fmt"
	"unsortedbytes.in/control/fileops"
	"github.com/Pallinder/go-randomdata"
)

const accountBalanceFile = "balance.txt"


func main(){

	accountBalance , err := fileops.GetFloatFromFile(accountBalanceFile, 1000.0)


	if err !=nil {
		fmt.Println("ERROR")
		fmt.Println(err)
		fmt.Println("---------------------------------")
		panic("Can't continue, Sorry.")
	}
	// fmt.Printf("Hello world")
	fmt.Println("Welcome to Go Bank! Hola Amigo")
	fmt.Println(randomdata.Email())

	for i:=0;i<2;i++{
		presentOptions()
		
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
			
			if depositAmount <=0 {
				fmt.Println("Invalid amount. Must be greater than 0.")
				continue
			}
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
			fmt.Println("Goodbye!, Thank you for your time")
			break
		}
		
		fileops.WriteFloatToFile(accountBalance, accountBalanceFile)
		
						
	}

	fmt.Println("Thanks for choosing our bank")

}


// func presentOptions(){
// 	fmt.Println()
// 	fmt.Println("What do you want to do?")
// 	fmt.Println("1. Check balance")
// 	fmt.Println("2. Deposit money")
// 	fmt.Println("3. Withdraw money")
// 	fmt.Println("4. Exit")
// }