import { render } from "solid-js/web";
import App from "./app.tsx";
import "./assets/app.css";
import "@fontsource/inter";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { AuthServiceClient } from "./lib/protoc/auth.client.ts";
import {
	EmployeeServiceClient,
	FileServiceClient,
	TaskCommentServiceClient,
	TaskServiceClient,
	UserServiceClient,
} from "./lib/protoc/employee_management.client.ts";

export const transport = new GrpcWebFetchTransport({
	baseUrl: import.meta.env.PUBLIC_GRPC_URL as string,
	interceptors: [
		{
			interceptUnary(next, method, request, metadata) {
				if (!metadata.meta) {
					metadata.meta = {};
				}

				const token = localStorage.getItem("Authorization");

				if (token) {
					metadata.meta.Authorization = token;
				}

				return next(method, request, metadata);
			},
		},
	],
});

export const authService = new AuthServiceClient(transport);
export const userService = new UserServiceClient(transport);
export const fileService = new FileServiceClient(transport);
export const employeeService = new EmployeeServiceClient(transport);
export const taskService = new TaskServiceClient(transport);
export const taskCommentService = new TaskCommentServiceClient(transport);

const root = document.getElementById("root");

if (!root) {
	throw new Error("Root element not found");
}

render(() => <App />, root);
