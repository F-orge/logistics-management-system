package main

import (
	"log"
	"log/slog"

	"github.com/F-orge/logistics-management-system/src/views/management"
	"github.com/F-orge/logistics-management-system/src/views/marketing"
	"github.com/labstack/echo/v4"
)

type Host struct {
	Echo *echo.Echo
}

func main() {

	hosts := map[string]*Host{}

	marketingSystem := marketing.New()
	managementSystem := management.New()

	// Marketing system
	hosts["www.localhost:8080"] = &Host{marketingSystem.Server()}
	hosts["management.localhost:8080"] = &Host{managementSystem.Server()}

	e := echo.New()

	slog.SetLogLoggerLevel(slog.LevelInfo)

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
