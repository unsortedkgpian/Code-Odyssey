package main

import "fmt"

func main(){
	var x [5]int
	x[4] =100
	fmt.Println(x)
	fmt.Println(x[4])

	var y[5]float64
	y[0] = 98
	y[1] = 45
	y[3] = 29
	y[2] = 61 
	y[4] = 84
	
	var total float64 =0;
	for i:=0;i<5;i++{
		total +=y[i]
	}
	fmt.Println("Average of :", y, "is ", total/float64(len(y)))

	for _, value :=range x{
		total +=float64(value)
	}
	fmt.Println(total/float64(len(x)))


	a:=[5]float64{192, 33.2, 42, 23,12,}
	fmt.Println(a)

	fmt.Println("\n\n Slices \n\n")
	
	s:= make([]float64, 5,10)
	ss:=s[1:3]
	// ss:=s[:3]
	// ss:=s[3:]
	// ss:=[0:len(arr)]
	fmt.Println(s)
	fmt.Println(ss)

	slice1 := []int{1,3,4, 2, 3, 53}
	slice2 := make([]int, 2,3)
	copy(slice2, slice1)
	fmt.Println(slice2)

	fmt.Println("\n\n\n Map \n\n\n")

	// var m map[string]int
	m:= make(map[string]int)
	fmt.Println("The lenght of map is now : ", len(m))
	m["key"] =109
	fmt.Println("The length of map is now : ",len(m))
	m["lock"] =802
	fmt.Println("The length of map is now : ", len(m))
	fmt.Println(m)
	delete(m, "key")
	fmt.Println(m)

	fmt.Println("\n\n\n")

	elements := make(map[string]string)
    elements["H"] = "Hydrogen"
	elements["He"] = "Helium"
	elements["Li"] = "Lithium"
	elements["Be"] = "Beryllium"
	elements["B"] = "Boron"
	elements["C"] = "Carbon"
	elements["N"] = "Nitrogen"
	elements["O"] = "Oxygen"
	elements["F"] = "Fluorine"
	elements["Ne"] = "Neon"
	fmt.Println(elements["Li"])
	fmt.Println(elements["uri"])

	fmt.Println("\nTesting the returning value\n")
	name1,ok1 := elements["Li"]
	name2, ok2 := elements["uri"]

	fmt.Println("name1: ",name1, "ok1: " , ok1 )
	fmt.Println("name2: ", name2, "ok2: ", ok2)

	if name, ok :=elements["O"]; ok {
			fmt.Println(name, ok)
	}

	ele := map[string]string{
		"H":"Hydrogen",
		"He":"Helium",
		"Li":"Lithium",
		"Be":"Beryllium",
	}


	ne := map[string]map[string]string{
		"H":map[string]string{
			"name":"Hydrogen",
			"state":"Gas",
		},
	}
	fmt.Println(ele)
	fmt.Println(ne)


	bv := []int{
		48,96,86,68,
		57,82,63,70,
		37,34,83,27,
		19,97, 9,17,
	}
	fmt.Println(bv)
	
	max :=0
	for _, rt :=range bv {
		if max < rt {
			max=rt
		}
	}
	fmt.Println("The maximum value of it is :", max)
}
