package main

import "fmt"


// -> Go is statically typed language-> all variable have pre defined data language

func main(){
	p,m:=fmt.Println("hello")
	fmt.Println(m)// -> nill what to make change
	fmt.Println(p)
	fmt.Println(1.01-0.99)
	fmt.Println("1 + 1  =", 1+1)
	fmt.Println("1 + 1 = ", 1.0 + 1.0)
	fmt.Println(len("Hello World"))
	fmt.Println("Hello World"[1])
	fmt.Println("Hello " + "World")
	fmt.Println(true && true)
	fmt.Println(true && false)
	fmt.Println(true || true)
	fmt.Println(true  || false)
	fmt.Println(!true)
	fmt.Println((true &&  false)  || (false && true) || (false && true) || !(false && false))
}
