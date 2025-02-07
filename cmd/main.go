package main

import (
	"log"

	"github.com/F-orge/logistics-management-system/web/plugins"
	"github.com/F-orge/logistics-management-system/web/plugins/authentication"
	humanresource "github.com/F-orge/logistics-management-system/web/plugins/human-resource"
	"github.com/labstack/echo/v4"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	e := echo.New()

	e.Static("/assets", "dist")

	// TODO: move this to be environment variable
	conn, err := grpc.NewClient("dns:localhost:8081", grpc.WithTransportCredentials(insecure.NewCredentials()))

	if err != nil {
		log.Fatal(err)
	}

	defer conn.Close()

	// TODO: create a extension for authentication
	extension := plugins.Extensions{}

	// register extension
	extension.Register(authentication.Authentication{})
	extension.Register(humanresource.HumanResource{})

	// build the extensions and bind it to the main echo instance
	extension.Build(e)

	if err := e.Start(":8080"); err != nil {
		log.Fatal(err)
	}
}
