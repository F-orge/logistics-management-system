package marketing

import (
	"github.com/labstack/echo/v4"
)

type Marketing struct {
}

func New() *Marketing {
	return &Marketing{}
}

func (m Marketing) Server(group *echo.Group) {

	group.GET("/", m.LandingRoute)
	group.GET("/about", m.AboutRoute)
	group.GET("/blogs", m.BlogsRoute)
	group.GET("/blogs/{id}", m.SpecificBlog)
}
