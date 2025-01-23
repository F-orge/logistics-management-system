package utils

import (
	"github.com/a-h/templ"
	"github.com/labstack/echo/v4"
)

func Render(comp templ.Component, c echo.Context) error {
	return comp.Render(c.Request().Context(), c.Response().Writer)
}
