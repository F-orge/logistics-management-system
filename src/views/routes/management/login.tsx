import { action } from "@solidjs/router";
import type { Component } from "solid-js";
import { Button } from "~/components/ui/button";
import { loginAction, LoginForm } from "~/features/authentication";

const LoginPage: Component<{}> = (props) => {
	return (
		<main class="h-screen p-4 flex flex-col justify-center items-center gap-10">
			<img src="/etmar-logo.png" class="h-8" alt="" />
			<h4 class="heading-4">Management Login</h4>
			<div class="w-full phone:w-1/2">
				<LoginForm action={loginAction}>
					<Button>Sign in</Button>
				</LoginForm>
			</div>
		</main>
	);
};

export default LoginPage;
