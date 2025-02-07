package humanresource

import "github.com/F-orge/logistics-management-system/web/plugins"

type HumanResource struct{}

func (h HumanResource) Name() string {
	return "Human Resource Management Information System"
}

func (h HumanResource) Path() string {
	return "/human-resource"
}

func (h HumanResource) Sidebar() []plugins.PageExtensionSidebar {
	return []plugins.PageExtensionSidebar{
		{
			Name: "Overview",
			Path: "/",
		},
	}
}

func (h HumanResource) Routes() []plugins.PageExtensionRoute {
	return []plugins.PageExtensionRoute{
		OverviewPage{},
	}
}
