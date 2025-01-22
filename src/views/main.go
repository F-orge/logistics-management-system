package main

import (
	"fmt"

	"github.com/F-orge/logistics-management-system/src/views/marketing"
)

func main() {
	fmt.Println("hello world")

	marketingSystem := marketing.New()

	marketingSystem.Start()
}
