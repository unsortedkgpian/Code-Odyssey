package main

import "fmt"

func main(){
	// creating map[]type
	m :=make(map[string]string)
	// settings  an elememt
	m["name"] = "golang"
	m["backend"] ="no-broker"

	// get an element
	fmt.Println(m["name"])
	fmt.Println(m["backend"])
	fmt.Println(m["work"])
	fmt.Println(len(m))

	// deleting
	//delete(m, "name")
	// clear
	clear(m)
	fmt.Println(m)

	ma:=map[string]int{"price":40, "nodeal":50}
	fmt.Println(ma)

	p,ok := ma["nodeal"]
	if ok{
		fmt.Println("its ok", ok)
	}else{
		fmt.Println("its not ok",p)
	}

}
