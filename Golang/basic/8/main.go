package main

import (
	"fmt"
	"sync"
)

func task(wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("Running goroutine")
}

func main() {
	var wg sync.WaitGroup

	wg.Add(5)
	go task(&wg)

	wg.Wait() // wait for goroutine to finish
}