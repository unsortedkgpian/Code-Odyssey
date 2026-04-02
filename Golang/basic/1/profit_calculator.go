package main

import(
	"fmt"
	"errors"
	"os"
)

func main(){
	fmt.Println("Profit Calculator")
	// var revenue , expenses , tax_rate float64 

	revenue , err := printAndScan("Revenue")
	if err != nil {
		fmt.Println(err)
		return
	}
	expenses, err := printAndScan("Expenses")
	if err != nil {
		fmt.Println(err)
		return
	}
	tax_rate, err := printAndScan("Tax rate")
	if err != nil {
		fmt.Println(err)
		return
	}
	

	EBT := calculateEBT(revenue, expenses)
	Profit := calculateProfit(EBT, tax_rate)

	ratio := EBT/Profit

	fmt.Println("Earning before tax : ", EBT)
	fmt.Println("Earning after tax : ", Profit)
	fmt.Println("The ratio : ", ratio)


	storeResults(EBT, Profit, ratio)
}

func storeResults(ebt, profit, ratio float64){
	results := fmt.Sprintf("EBT: %.1f\nProfit:%.1f\nRatio:%.3f\n", ebt, profit, ratio)
	os.WriteFile("results.txt", []byte(results), 0644)
}

func printAndScan(value string) (v float64, err error){
	fmt.Print(value, " :")
	fmt.Scan(&v)

	if v <=0 {
		return 0, errors.New("Value must be a postive number.")
	}
	return v, nil
}

func calculateEBT(revenue, expenses float64)(EBT float64){
	EBT = revenue - expenses
	return
}

func calculateProfit(EBT , tax_rate float64)(profit float64){
	profit = (EBT)*(1-tax_rate/100)
	return 
}