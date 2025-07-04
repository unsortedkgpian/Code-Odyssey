package main
import "fmt"

// for -> only  consturct in go for looping
func main(){
	 // while looping
	var i int64 =1
	for i<=3 {
		fmt.Println(i)
		i = i+1
	}
	// infinite loop 
	for {
		println(i)
		i=i*i
		if i == 0{
			break
		}
	}

	//classic for loop
	for i:=0;i<3;i++{
		println(i);
	}

	for a:= range 5 {
		fmt.Println(a)
	}
}
