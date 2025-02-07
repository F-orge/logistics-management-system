package authentication

import "github.com/F-orge/logistics-management-system/web/plugins"

type Authentication struct{}

func (a Authentication) Name() string {
	return "Authentication"
}

func (a Authentication) Path() string {
	return "/auth"
}

func (a Authentication) Sidebar() []plugins.PageExtensionSidebar {
	return []plugins.PageExtensionSidebar{}
}

func (a Authentication) Routes() []plugins.PageExtensionRoute {
	return []plugins.PageExtensionRoute{
		LoginPage{},
	}
}
