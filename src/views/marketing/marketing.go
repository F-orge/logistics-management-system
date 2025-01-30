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
	group.GET("/", m.MarketingLandingPageHandler)
	group.GET("/about", m.MarketingAboutPageHandler)
	group.GET("/services", m.MarketingServicesHandler)
}
