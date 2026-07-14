package main

import (
	"net/http"
	"log"
)

func main(){
	//fmt.Println("Hello World")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
