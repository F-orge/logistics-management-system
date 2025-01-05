import type { Component } from "solid-js";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "../components/ui/card";
import { Button } from "../components/ui/button";
import Input from "../components/ui/input";
import { Label } from "../components/ui/label";
import {
	TextField,
	TextFieldInput,
	TextFieldLabel,
} from "~/components/ui/text-field";
import { action, redirect } from "@solidjs/router";
import { authService } from "~/entry-client";
import { showToast } from "~/components/ui/toast";
import { LoginForm, loginAction } from "~/features/authentication";

const EmployeeManagementLoginPage: Component<{}> = (props) => {
	return (
		<main class="flex flex-col items-center justify-center h-screen gap-5">
			<div class="w-full max-w-2xl">
				<h1>Etmar Logistics</h1>
				<LoginForm action={loginAction}>
					<div>
						<Button type="submit" class="w-full">
							Login
						</Button>
					</div>
				</LoginForm>
			</div>
		</main>
	);
};

export default EmployeeManagementLoginPage;
