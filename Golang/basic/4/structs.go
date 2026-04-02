package main

import (
	"fmt"
	"unsortedbytes.in/structs/user"
)




func main(){
	// fmt.Println("Hello World!")
	userFirstName := getUserData("Please enter your first name: ")
	userLastName := getUserData("Please enter your last name: ")
	userBirthdate := getUserData("Please enter your birthdate (DD/MM/YYYY):")
	
	
	// var appUser user 
	
	appUser, err := user.New(userFirstName, userLastName, userBirthdate)

	if err != nil {
		fmt.Println(err)
		return 
	}

	appUser.OutputUserDetailsMethods()
	appUser.ClearUserNameMethods("Indian")

	// fmt.Println(firstName, lastName, birthdate)

	// outputUserDetails(appUser)

	admin := user.NewAdmin("ok@ok.com", "password")
	admin.OutputUserDetailsMethods()
}


// func outputUserDetails( u *user.User){
// 	fmt.Println(u.firstName, u.lastName, u.birthDate)
// }

func getUserData(promtText string) string {
	fmt.Print(promtText)
	var value string
	fmt.Scanln(&value)
	return value
}