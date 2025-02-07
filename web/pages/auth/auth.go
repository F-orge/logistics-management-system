package auth

import (
	"github.com/F-orge/logistics-management-system/web/proto/auth"
	"github.com/labstack/echo/v4"
	"google.golang.org/grpc"
)

type AuthHandler struct {
	GrpcClient auth.AuthServiceClient
}

func New() *AuthHandler {
	return &AuthHandler{}
}

func (a *AuthHandler) Build(conn grpc.ClientConnInterface, group echo.Group) {
	a.GrpcClient = auth.NewAuthServiceClient(conn)
	group.GET("/login", a.ShowLogin)
	group.POST("/login", a.LoginAction)
}
