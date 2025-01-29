package humanresource

import "github.com/labstack/echo/v4"

type HumanResource struct{}

func New() *HumanResource {
	return &HumanResource{}
}

func (h *HumanResource) Server(e *echo.Group) {
	// views
	e.GET("/human-resource", h.OverviewPageHandler)
	e.GET("/human-resource/employees", h.EmployeePageHandler)
	e.GET("/human-resource/departments", h.DepartmentsPageHandler)
	e.GET("/human-resource/tasks", h.TasksPageHandler)
}
