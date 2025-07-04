package main
import "fmt"

func main(){
	// Slices -> dynamic arrays
	// most used in go 

	var nums []bool
	// uninitilized slice is nil (null from js)
	fmt.Println("Before anthing")
	fmt.Println(nums == nil)
	fmt.Println(len(nums))

	var num1 = make([]bool, 10, 20)

	for i:=range 50 {
		num1 = append(num1, true) 
		i=i+1
	}

	fmt.Println(num1==nil)
	fmt.Println(len(num1))
	fmt.Println(cap(num1))
	fmt.Println(num1)


	var numso =[]int{1,2,3}
	
	fmt.Println(numso[0:2])
	fmt.Println("ao")


	var demo = [][]int{{1,343,5}, {24,9}, {53,98,23}}

	fmt.Println(demo)

}
