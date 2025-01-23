package marketing

import (
	"github.com/labstack/echo/v4"
)

type Marketing struct {
	server *echo.Echo
}

func New() *Marketing {
	return &Marketing{
		server: echo.New(),
	}
}

func (m *Marketing) Server() *echo.Echo {

	m.server.GET("/", m.LandingRoute)
	m.server.GET("/about", m.AboutRoute)
	m.server.GET("/blogs", m.BlogsRoute)
	m.server.GET("/blogs/{id}", m.SpecificBlog)

	return m.server
}
