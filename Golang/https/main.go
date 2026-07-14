package main 

import (
	"fmt"
	"os"
)

func main(){
	fmt.Println("I hope I get the job!")
	file, _ := os.Open("messages.txt")
	fmt.Println(file)
}
