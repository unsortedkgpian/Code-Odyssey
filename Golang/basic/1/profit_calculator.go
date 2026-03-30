package main

import(
	"fmt"
)

func main(){
	fmt.Println("Profit Calculator")
	var revenue , expenses , tax_rate float64 

	revenue = printAndScan("Revenue")
	expenses = printAndScan("Expenses")
	tax_rate = printAndScan("Tax rate")
	

	EBT := calculateEBT(revenue, expenses)
	Profit := calculateProfit(EBT, tax_rate)

	ratio := EBT/Profit

	fmt.Println("Earning before tax : ", EBT)
	fmt.Println("Earning after tax : ", Profit)
	fmt.Println("The ratio : ", ratio)
}

func printAndScan(value string) (v float64){
	fmt.Print(value, " : ")
	fmt.Scan(&v)
	return
}

func calculateEBT(revenue, expenses float64)(EBT float64){
	EBT = revenue - expenses
	return
}

func calculateProfit(EBT , tax_rate float64)(profit float64){
	profit = (EBT)*(1-tax_rate/100)
	return 
}