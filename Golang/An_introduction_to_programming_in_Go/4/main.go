package main

import "fmt"

var a string="ok"

func main(){
	var x string = "hello world"
	x = "Hello World"
	fmt.Println(x)
	x = "nothing"
	fmt.Println(x)
	x = x+ "pj"
	fmt.Println(x)
	x+=" ok"
	fmt.Println(x)

	var y string ="ok world"
	fmt.Println(x ==y )
	z:="no work"
	fmt.Println(z)
	fmt.Println(a)
	// Const with it 
	const k string = "kelvin constant"
	//k="net prime"
	fmt.Println(k)
	//	var (
	//	pi=3.14
	//	pickacu="Pokemon"
	//)
	const (
		pi=3.14
		pickacu = "Pokemon"
	)
	fmt.Println("PI is :", pi)
	fmt.Println("Pickacu is :",pickacu)
	// An Example Program 
	fmt.Println("Enter a number: ")
	var input float64
	fmt.Scanf("%f",&input)
	fmt.Println("Output:",input*2)

	// Program to Fahrenheit to Celcius
	fmt.Println("Enter the value of Fahrenheit:")
	var inputf float64
	fmt.Scanf("%f", &inputf)
	celcius := (inputf-32)*(5.0/9.0)
	fmt.Println("The value of Celcius:", celcius)
	
	// Program to convert feat into meter
	fmt.Println("Enter the length into feat:")
	var feat float64
	fmt.Scanf("%f", &feat)
	meter := feat*0.3048 
	fmt.Println("Enter lentth in meter is :",meter)
}

func f(){
	//fmt.Println(x)
}
