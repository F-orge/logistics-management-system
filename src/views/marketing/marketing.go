package marketing

import (
	"log"

	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/core"
)

type Marketing struct {
	pb *pocketbase.PocketBase
}

func New() *Marketing {
	return &Marketing{
		pb: pocketbase.New(),
	}
}

func (m *Marketing) Start() {

	m.pb.OnServe().BindFunc(func(e *core.ServeEvent) error {

		e.Router.GET("/", m.LandingRoute)
		e.Router.GET("/about", m.AboutRoute)
		e.Router.GET("/blogs", m.BlogsRoute)
		e.Router.GET("/blogs/{id}", m.SpecificBlog)

		return e.Next()
	})

	if err := m.pb.Start(); err != nil {
		log.Fatal(err)
	}
}
