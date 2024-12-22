import { render } from "solid-js/web";
import App from "./app.tsx";
import "./app.css";
import "@fontsource/inter";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

export const transport = new GrpcWebFetchTransport({
	baseUrl: import.meta.env.PUBLIC_GRPC_URL as string,
});

const root = document.getElementById("root");

if (!root) {
	throw new Error("Root element not found");
}

render(() => <App />, root);
