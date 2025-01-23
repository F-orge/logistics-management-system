package main

import (
	"log"

	"github.com/F-orge/logistics-management-system/src/views/marketing"
	"github.com/labstack/echo/v4"
)

type Host struct {
	Echo *echo.Echo
}

func main() {

	hosts := map[string]*Host{}

	marketingSystem := marketing.New()

	// Marketing system
	hosts["www.localhost:8080"] = &Host{marketingSystem.Server()}

	e := echo.New()

	e.Static("/assets", "./src/views/assets")

	e.Any("/*", func(c echo.Context) (err error) {
		req := c.Request()
		res := c.Response()
		host := hosts[req.Host]
		if host == nil {
			err = echo.ErrNotFound
		} else {
			host.Echo.ServeHTTP(res, req)
		}
		return
	})

	if err := e.Start(":8080"); err != nil {
		log.Fatal(err)
	}
}
