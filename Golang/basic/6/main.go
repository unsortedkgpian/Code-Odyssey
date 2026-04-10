package main

import (
	"fmt"
	"unsortedbytes.in/project-struct/note"
	"unsortedbytes.in/project-struct/todo"
	"bufio"
	"os"
	"strings"
)

type saver interface {
	Save() error 
}


type displayer interface {
	Display()
}

type outputtable interface {
	Save() error
	Display()
}


func main(){
	
	title, content := getNoteData()

	todoText := getUserInput("Todo text: ")

	todo, err := todo.New(todoText)

	if err!= nil{
		fmt.Println(err)
		return 
	}

	todo.Display()
	err = saveData(todo)
	if err!= nil {
		fmt.Println(err)
		return
	}
	


	userNote, err := note.New(title, content)


	if err != nil {
		fmt.Println(err)
		return
	}

	userNote.Display()
	saveData(userNote)

	anyvalueOutput("5")
}


func outputData(data outputtable){
	data.Display()
	saveData(data)
}

func anyvalueOutput(value interface{} ){
	fmt.Printf("Type: %T, Value: %v \n", value, value)

	switch value.(type) {
	case int:
		fmt.Println("Type got messedup : int ", value)
	
	case float64:
		fmt.Println("I dnotkdkf")

	default:
		fmt.Println("we don't know what to do with you")
	}


	intValue, ok := value.(int)

	if ok {
		fmt.Println("Integers: ", intValue)
	}
}

// func getTodoData() string {
// 	text := getUserInput("Todo text: ")
// 	return text
// }

func saveData(data saver) error {
	err:= data.Save()


	if err != nil {
		fmt.Println("Saving the data failed.")
		return err
	}

	fmt.Println("Saving the data succeeded!")
	return nil
}

func getNoteData()(string, string ){

	title := getUserInput("Note title: ")


	content := getUserInput("Note content: ")

	return title, content
}


func getUserInput(prompt string) string{
	fmt.Printf("%v ", prompt)


	reader := bufio.NewReader(os.Stdin)

	text, err := reader.ReadString('\n')

	if err != nil {
		return ""
	}

	text = strings.TrimSuffix(text, "\n")
	text = strings.TrimSuffix(text, "\r")
	return text
}