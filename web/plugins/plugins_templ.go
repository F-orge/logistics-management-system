// Code generated by templ - DO NOT EDIT.

// templ: version: v0.3.833
package plugins

//lint:file-ignore SA4006 This context is only used if a nested component is present.

import "github.com/a-h/templ"
import templruntime "github.com/a-h/templ/runtime"

import "github.com/labstack/echo/v4"

import "github.com/F-orge/logistics-management-system/web/utils"

// Example:
// Note: sidebar
// Human resource // name of the extension
// 	- Employee // has a equivalent path of `/human-resource/employee/`
// 	- Department
//	- Tasks
//	- Files

// route structure: /extension-name/pages/action
// note: every extension route must have the following.
type PageExtension interface {
	Name() string                    // note: name of the extension
	Path() string                    // note: should be /extension-name
	Sidebar() []PageExtensionSidebar // note: this sidebar is intended to be rendered depending on the layout
	Routes() []PageExtensionRoute    // note: collection of pages inside a extension
}

type PageExtensionSidebar struct {
	Name string // name of the `a` tag that will be rendered
	Path string // href
	Icon string // note: we should use lucide-icons
}

// sidebarRoute - for sidebar navigation
type PageExtensionRoute interface {
	Path() string
	Page(c echo.Context) templ.Component // HTTP: GET method for rendering the page. note: this page will always be path as `/`
	Actions(group echo.Group)            // note: this will be used to bind other functions that the current page needed in order to work. example: store employee information.
}

type Extensions struct {
	PExtensions []PageExtension
}

func (e *Extensions) Register(extension PageExtension) {
	e.PExtensions = append(e.PExtensions, extension)
}

func (e Extensions) Build(ec *echo.Echo) {
	for _, ex := range e.PExtensions {
		group := ec.Group(ex.Path()) // this will create an extension. example: /human-resource
		for _, route := range ex.Routes() {
			pageGroup := group.Group(route.Path())
			e.bindRoute(pageGroup, route)
		}
	}
}

func (e Extensions) bindRoute(group *echo.Group, route PageExtensionRoute) {
	group.GET("/", func(c echo.Context) error { // binding the page route
		return utils.Render(c, 200, route.Page(c))
	})
	// bind the actions
	route.Actions(*group)
}

var _ = templruntime.GeneratedTemplate
