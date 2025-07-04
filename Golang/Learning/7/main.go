package main
import "fmt"
import "time"

func main(){
	// switch expression 


	i:=2
	switch i {
	case 1:
		fmt.Println("one")
	case 2:
		fmt.Println("two")
	case 3:
		fmt.Println("three")
	default:
		fmt.Println("others")
	}
	
	switch time.Now().Weekday(){
	case time.Saturday, time.Sunday:
		fmt.Println("Weekend")
	default:
		fmt.Println("Workday")
	}

	whoAmI := func(i interface{}){
		switch t:=i.(type){
		case int:
			fmt.Println("Its the integers!")
		case float64:
			fmt.Println("Its Float")
		case string:
			fmt.Println("Its a string")
		case bool:
			fmt.Println("Its boolean")
		default:
			fmt.Println("i don't know", t)
		}
	}

	whoAmI("golang")
	whoAmI(3.23)
	whoAmI(2)
	whoAmI(2.3)
}
