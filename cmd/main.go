package main

import (
	"fmt"
	"log"

	"github.com/F-orge/logistics-management-system/web/pages/auth"
	"github.com/labstack/echo/v4"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	e := echo.New()

	e.Static("/assets", "dist")

	conn, err := grpc.NewClient("dns:localhost:8081", grpc.WithTransportCredentials(insecure.NewCredentials()))

	if err != nil {
		fmt.Println("Failed to connect to server:", err)
		return
	}

	defer conn.Close()

	auth.New().Build(conn, *e.Group("/auth"))

	if err := e.Start(":8080"); err != nil {
		log.Fatal(err)
	}
}
