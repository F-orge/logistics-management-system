package management

import (
	"github.com/F-orge/logistics-management-system/src/views/utils"
	"github.com/labstack/echo/v4"
)

type HumanResource struct {
	server *echo.Echo
}

func New() *HumanResource {
	return &HumanResource{
		server: echo.New(),
	}
}

func (h *HumanResource) Server() *echo.Echo {

	h.server.GET("/login", func(c echo.Context) error {
		return utils.Render(h.LoginPage(c), c)
	})
	h.server.GET("/", func(c echo.Context) error {
		return utils.Render(h.HomePage(c), c)
	})
	h.server.POST("/login", h.LoginActionRoute)

	return h.server
}
