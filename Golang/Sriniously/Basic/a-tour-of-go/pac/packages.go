package main

import (
	"fmt"
	"math"
	"math/rand"
)

// func add(x int, y int) int {
func add(x, y int) int {
	return x+y
}

func swap(x, y string) (string, string){
	return y, x
}

func split(sum int)(x, y int){
	x = sum*4/9
	y = sum -x
	return 
}

var c, python, java bool

var i2, j2 int =  1, 2

const Pi = 3.1423


const (
	// Bit shift
	Big = 1<<100
	Small = Big >>99
)

func needInt( x int) int{
	return x*10 + 1
}

func needFloat(x float64) float64{
	return x*0.1
}

func main() {
	fmt.Println("My favorite number is", rand.Intn(100))
	fmt.Printf("Now you have %g problems.\n", math.Sqrt(7))
	fmt.Println(math.Pi)

	fmt.Println(add(45, 23))

	a, b := swap("Hello", "World")
	fmt.Println(a, b)


	// Naked return
	fmt.Println(split(17))

	// Variables 
	var i int
	fmt.Println(i, c, python,java)

	var c2, python2, java2 = true, false, "no!"
	fmt.Println(i2, j2, c2, python2, java2)

	var i3, j3 int = 1, 2
	k:= 3
	c3,  python3, java3 := true, false, "no!"
	fmt.Println(i3, j3, k, c3 , python3, java3)


	var i4 int
	var f4 float64
	var b4 bool 
	var s4 string 
	fmt.Printf("%v %v %v %q\n",i4, f4, b4, s4 )

	var x5, y5 int = 3, 4
	var f5 float64 = math.Sqrt(float64(x5*x5 + y5*y5) + 8.8)
	var z5 uint = uint(f5)
	fmt.Println(x5, y5,z5, f5)


	// Type inference
	// %v -> value
	// %T -> Type

	v:= 42 + 32i
	fmt.Println(v)
	v1 :=3 +2i 
	v2 := 4 - 5i
	fmt.Println(v1*v2)
	fmt.Printf("v is of type %T\n", v)

	const World = "世界"
	fmt.Println("Hello", World)
	fmt.Println("Happy", Pi, "Day")

	const Truth = true 
	fmt.Println("Go rules?", Truth)

	// fmt
	fmt.Println(needInt(Small))
	fmt.Println(needFloat(Small))
	fmt.Println(needFloat(Big))
	fmt.Println(needInt(Big))// overflows



}
