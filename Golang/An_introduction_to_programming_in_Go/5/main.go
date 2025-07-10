package main

import "fmt"

func main(){
    fmt.Println(1)
    fmt.Println(2)
    fmt.Println(3)
    fmt.Println(4)
    fmt.Println(5)
    fmt.Println(6)
    fmt.Println(7)
    fmt.Println(8)
    fmt.Println(9)
    fmt.Println(10)

    fmt.Println("\n \n Trying other way\n\n")

	fmt.Println("1 2 3 4 5 6 7 8 9 10")

	fmt.Println("\n \n For Loop \n \n")

	i:=1
	for i<=10 {
		fmt.Println(i)
		i+=2
	}
	
//	for i>0{
//		fmt.Println("Infinte loop : ",i)
//		i+=1
//	}


	for j:=1;j<=10;j++{
		fmt.Println("for loop advanced : ", j)
	}


	fmt.Println("\n\n\n\n If condition \n")

	for k:=1;k<=10;k++{
		if k%2 ==0 {
			fmt.Println(k," Even")
		}else {
			fmt.Println(k, " Odd")
		}
	}

	fmt.Println("\n\n\n Switch \n")
	k:=8	
	switch k {
	case 0: 
		fmt.Println("Its value is 0")
	case 1:
		fmt.Println("Its value is 1")
	case 2:
		fmt.Println("Its value is 2")
	case 3:
		fmt.Println("Its value is 3")
	case 4:
		fmt.Println("Its vlaue is 4")
	case 5:
		fmt.Println("Its value is 5")
	case 6:
		fmt.Println("Its value is 6")
	case 7:
		fmt.Println("Its value is 7")
	case 8:
		fmt.Println("Its value is 8")
	case 9:
		fmt.Println("Its value is 9")
	case 10:
		fmt.Println("Its value is 10")
	}

	for i:=1;i<=100;i++{
		if i%3==0 {
			fmt.Println("It is divisble by 3 : ",i)
		}
	}
	for i:=1;i<=100;i++{
		if i%3 == 0 && i%5 ==0{
			fmt.Println("FizzBuzz")
		}else if i%5 ==0 {
			fmt.Println("Buzz")
		}else if i%3 ==0 {
			fmt.Println("Fizz")
		}else{
			fmt.Println(i)
		}
	}
}
