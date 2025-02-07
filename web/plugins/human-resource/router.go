package humanresource

import (
	"github.com/F-orge/logistics-management-system/web/plugins"
	"github.com/labstack/echo/v4"
)

type HumanResource struct{}

func (h HumanResource) Name() string {
	return "Human Resource management"
}

func (h HumanResource) Pages() []plugins.PluginPage {
	return []plugins.PluginPage{
		OverviewPage{},
		AttendancePage{},
	}
}

func (h HumanResource) Build(e *echo.Group) {
	for _, item := range h.Pages() {
		g := e.Group(item.Path())
		item.Routes(g)
	}
}
