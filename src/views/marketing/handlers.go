package marketing

import (
	"github.com/F-orge/logistics-management-system/src/views/utils"
	"github.com/labstack/echo/v4"
)

func (m *Marketing) LandingRoute(c echo.Context) error {
	return utils.Render(LandingPage(), c)
}

func (m *Marketing) AboutRoute(c echo.Context) error {
	return c.String(200, "About page")
}

func (m *Marketing) BlogsRoute(c echo.Context) error {
	return c.String(200, "Blogs page")
}

func (m *Marketing) SpecificBlog(c echo.Context) error {
	return c.String(200, "Specific blogs")
}
