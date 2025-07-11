package main

import "fmt"
var x int = 38

func main(){
	array := []float64{98, 93, 77, 82, 83}
	fmt.Println(average(array))
	f(x)
	x, y:=multi()
	fmt.Println( x, y)

	fmt.Println("\n\n\n Vardiadic Function\n")

	fmt.Println("The sum of 4 and 5 is :", add(4,5))
	fmt.Println("The sum of 3, 4, 5, 2, 1, 1, 3,4 is :" , add(3,4,5,2,1,1,3,4))

	p :=1
	increment := func()int{
		p++
		return p
	}
	fmt.Println(increment())
	fmt.Println(increment())
	fmt.Println(increment())
	fmt.Println(increment())
	fmt.Println(increment())
	fmt.Println("\n\n", factorial(5))
}

func factorial (n uint) uint{
	if n == 0 || n ==1 {
		return 1
	}
	return n*factorial(n-1)
}

func makeEverGenerator() func() uint{
	i := uint(0)
	return func()(ret uint){
		ret = i 
		i+=2
		return
	}
}

func add(args ...int) int{
	total:= 0
	for _, v := range args{
		total += v
	}
	return total
}


func average(array []float64) float64{
	total :=0.0
	for _,v := range array {
		total +=v 
	}
	return  total/float64(len(array))
}

func f(x int){
	fmt.Println(x)
}

func multi () (int, int){
	return 4, 2
}
