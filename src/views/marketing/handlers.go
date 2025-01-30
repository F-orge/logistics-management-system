package marketing

import (
	"github.com/labstack/echo/v4"
)

func (m *Marketing) AboutRoute(c echo.Context) error {
	return c.String(200, "About page")
}

func (m *Marketing) BlogsRoute(c echo.Context) error {
	return c.String(200, "Blogs page")
}

func (m *Marketing) SpecificBlog(c echo.Context) error {
	return c.String(200, "Specific blogs")
}
