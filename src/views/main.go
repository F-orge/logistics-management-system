package main

import (
	"log"
	"log/slog"

	"github.com/F-orge/logistics-management-system/src/views/management"
	"github.com/F-orge/logistics-management-system/src/views/marketing"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"golang.org/x/time/rate"
)

type Host struct {
	Echo *echo.Echo
}

func main() {

	// echo server
	e := echo.New()

	managementSystem := e.Host("management.localhost:8080")
	marketingSystem := e.Host("www.localhost:8080")
	management.New().Server(managementSystem)
	marketing.New().Server(marketingSystem)
	managementSystem.Static("/assets", "./src/views/assets")
	marketingSystem.Static("/assets", "./src/views/assets")

	// set body limit to 1GB
	e.Use(middleware.BodyLimit("1G"))
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		// only allow PUBLIC_ENV_HOST_NAME only
		AllowOrigins: []string{
			"www.localhost:8080",
			"management.localhost:8080",
		},
		// TODO: add more headers if needed
		AllowHeaders: []string{
			echo.HeaderOrigin, echo.HeaderContentType, echo.HeaderAccept,
			echo.HeaderAuthorization,
		},
		AllowCredentials: true,
	}))
	e.Use(middleware.CSRFWithConfig(middleware.CSRFConfig{
		TokenLookup: "form:_csrf",
	}))
	e.Use(middleware.Decompress())
	e.Use(middleware.Gzip())
	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "request_id=${id} remote_ip=${remote_ip} method=${method}, uri=${uri}, status=${status}\n",
	}))
	e.Use(middleware.RateLimiter(middleware.NewRateLimiterMemoryStore(rate.Limit(20))))
	e.Use(middleware.Recover())
	e.Use(middleware.RequestID())
	e.Use(middleware.Secure())
	e.Use(middleware.Timeout())

	slog.SetLogLoggerLevel(slog.LevelInfo)

	if err := e.Start(":8080"); err != nil {
		log.Fatal(err)
	}
}
