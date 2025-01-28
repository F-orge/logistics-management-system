package management

import (
	"log/slog"

	"github.com/F-orge/logistics-management-system/src/views/utils"
	"github.com/labstack/echo/v4"
)

type HumanResource struct {
}

func New() *HumanResource {
	return &HumanResource{}
}

func (h *HumanResource) Server(group *echo.Group) {

	group.GET("/login", func(c echo.Context) error {
		// generate csrf token
		slog.Info(c.Get("csrf").(string))

		return utils.Render(h.LoginPage(c), c)
	})
	group.GET("/", func(c echo.Context) error {
		return utils.Render(h.HomePage(c), c)
	})
	group.GET("/logout", func(c echo.Context) error {
		// remove authorization cookie
		cookie, err := c.Cookie("Authorization")

		if err != nil {
			return err
		}

		// make it empty
		cookie.Value = ""

		c.SetCookie(cookie)

		return c.Redirect(301, "/login")
	})
	group.POST("/login", h.LoginActionRoute)

}
