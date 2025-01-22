package marketing

import "github.com/pocketbase/pocketbase/core"

func (m *Marketing) LandingRoute(e *core.RequestEvent) error {
	return e.String(200, "Landing page")
}

func (m *Marketing) AboutRoute(e *core.RequestEvent) error {
	return e.String(200, "About page")
}

func (m *Marketing) BlogsRoute(e *core.RequestEvent) error {
	return e.String(200, "Blogs page")
}

func (m *Marketing) SpecificBlog(e *core.RequestEvent) error {
	return e.String(200, "Specific blogs")
}
