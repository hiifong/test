package main

import (
	"fmt"
	"os"
	"time"

	"typst-go"
)

func main() {
	now := time.Now()
	fmt.Println("Hello World")

	fmt.Printf("8 + 10 = %d\n", typst.Add(8, 10))

	content, err := os.ReadFile("./README.md")
	if err != nil {
		panic("Read file Error")
	}

	fmt.Println(typst.Render(string(content)))

	fmt.Println(time.Since(now))
}
