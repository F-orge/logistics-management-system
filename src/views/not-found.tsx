import type { Component } from "solid-js";
import { Button } from "./components/ui/button";
import { ArrowLeft } from "lucide-solid";
import { A } from "@solidjs/router";

const NotFoundPage: Component<{}> = (props) => {
	return (
		<main class="h-screen flex flex-col justify-center items-center gap-2.5 px-4">
			<span class="text-primary">404</span>
			<h1 class="heading-1">Page not found</h1>
			<p class="paragraph lead">
				Sorry, we couldn’t find the page you’re looking for.
			</p>
			<div class="">
				<Button variant={"link"} as={A} href="/">
					<ArrowLeft />
					Return Home
				</Button>
			</div>
		</main>
	);
};

export default NotFoundPage;
