package fileops

import(
	"os"
	"errors"
	"strconv"
	"fmt"
)


func GetFloatFromFile(fileName string, defaultValue float64) (float64, error) {
	data, err := os.ReadFile(fileName)

	if err != nil {
		return defaultValue, errors.New("Failed to find file.")
	}
	valueText := string(data)
	value, err := strconv.ParseFloat(valueText, 64)

	if err != nil{
		return defaultValue, errors.New("Failed to parse stored balance value.")
	}
	return value, nil
}

func WriteFloatToFile(value float64 , fileName string){
	valueText := fmt.Sprint(value)
	os.WriteFile("balance.txt", []byte(valueText),0644)
}
