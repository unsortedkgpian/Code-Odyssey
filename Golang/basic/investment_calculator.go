package main

import (
	"math"
	"fmt"
)


func main(){
	const inflationRate float64 = 2.5	
	// fmt.Print("Hello World!\n")
	// var investmentAmount, years float64 = 1000 , 10
	// investmentAmount, years := 1000.0, 10.0
	var investmentAmount float64
	// years:= 10.0
	var years float64
	// expectedReturnRate  := 5.5
	var expectedReturnRate float64
	// var years float64 = 10
	fmt.Print("Investment Amount: ")
	fmt.Scan(&investmentAmount);
	fmt.Print("No of years: ")
	fmt.Scan(&years)
	fmt.Print("Expected Return Rate: ")
	fmt.Scan(&expectedReturnRate)

	futureValue, futureRealValue := calculateFutureValue(investmentAmount, expectedReturnRate, years, inflationRate)
	// futureRealValue := futureValue/math.Pow((1+ inflationRate/100), years)


	formatedFV := fmt.Sprintf("Future Value: %.2f",futureValue)
	formatedRFV := fmt.Sprintf("Future Real Value : %.2f",futureRealValue)
	fmt.Println(formatedFV)
	fmt.Println(formatedRFV)

	outputText("Just for fun")
}


func outputText(text string){
	fmt.Print(text)
}


func calculateFutureValue(investmentAmount, expectedReturnRate, years, inflationRate float64)(fv float64, frv float64){
	fv = investmentAmount*math.Pow(1+expectedReturnRate/100, years)
	frv =  fv/math.Pow(1+inflationRate/100, years)
	return 
}