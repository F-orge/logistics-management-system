import { render } from "solid-js/web";
import App from "./app.tsx";
import "./app.css";
import "@fontsource/inter";

const root = document.getElementById("root");

if (!root) {
	throw new Error("Root element not found");
}

render(() => <App />, root);
