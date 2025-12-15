package main

import (
	"fmt"
	"math"
)


func sqrt( x float64) string{
	if x < 0 {
		return sqrt(-x) + "i"
	}
	return fmt.Sprint(math.Sqrt(x))
}

func pow( x, n ,lim float64) float64{
	if v:= math.Pow(x, n); v < lim{
		return v
	}
	return lim
}

func pow2(x, n , lim float64) float64{
	if v:= math.Pow(x, n); v<lim {
		// fmt.Println("Ok")
		return v
	}else{
		fmt.Printf("%g >= %g\n", v, lim)
	}

	return lim
}

func sqrt2(x float64) string{
	if x< 0{
		return sqrt2(-x) + "i"
	}

	return fmt.Sprint(math.Sqrt(x))
}

func main(){
	sum :=0
	for i :=0 ;i<10;i++{
		sum+=i
	}

	fmt.Println(sum)

	sum2 :=1
	for ; sum2<1000;{
		sum2 += sum2 
	}

	fmt.Println(sum2)


	sum3 :=1
	for sum3 < 1000{
		sum3 +=sum3
	}


	fmt.Println(sum3)
	// for{
	// 	fmt.Println(sum3)
	// 	sum3+=100000000000
	// }


	fmt.Println(sqrt(2), sqrt(-4))


	fmt.Println(
		pow(3,2,10),
		pow(3,3,10),
	)

	fmt.Println(
		pow2(3,2,10),
		pow2(3,3,20),
	) 
}