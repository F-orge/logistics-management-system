package management

import (
	"log/slog"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
)

func (h *HumanResource) LoginActionRoute(c echo.Context) error {

	formEmail := c.FormValue("email")
	formPassword := c.FormValue("password")

	// verify
	slog.Info(formEmail + " " + formPassword)

	// if okay send cookie and redirect to /
	cookie := new(http.Cookie)
	cookie.Name = "Authorization"
	cookie.Value = "please change me!"
	cookie.Expires = time.Now().Add(24 * time.Hour) // we set it to one day
	c.SetCookie(cookie)

	return c.Redirect(308, "/")
}
