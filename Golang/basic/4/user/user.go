package user

import (
	"time"
	"errors"
	"fmt"
)

type User struct{
	firstName string 
	lastName string
	birthDate string 
	age int
	createdAt time.Time
}

type Admin struct{
	email string
	password string 
	// User User
	User
}

func New(userFirstName, userLastName, userBirthdate string) (*User, error) {

	if userFirstName == "" || userLastName == "" || userBirthdate == "" {
		return nil, errors.New("a empty string is passed")
	}

	return &User{
		firstName:userFirstName,
		lastName: userLastName,
		birthDate: userBirthdate,
		createdAt: time.Now(),
	}, nil
}

func NewAdmin(email, password string) Admin{
	return Admin{
		email: email,
		password:password,
		User: User{
			firstName: "ADMIN",
			lastName: "Admin",
			birthDate:"---",
			createdAt: time.Now(),
		},
	}
}

func (u User) OutputUserDetailsMethods(){
	fmt.Println(u.firstName, u.lastName, u.birthDate)
}

func (u *User) ClearUserNameMethods(s string){
	u.firstName = s

}