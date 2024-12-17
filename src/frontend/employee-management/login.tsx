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

const EmployeeManagementLoginPage: Component<{}> = (props) => {
	return (
		<main class="flex flex-col items-center justify-center h-screen gap-5">
			<Card class="w-1/2 border-none">
				<CardHeader>
					<div class="flex flex-row items-center justify-between">
						<div class="space-y-1.5">
							<CardTitle>Employee Management</CardTitle>
							<CardDescription>
								Login to access employee management features
							</CardDescription>
						</div>
						<img height={90} width={90} src="/etmar-logo.png" alt="" />
					</div>
				</CardHeader>
				<form action="">
					<CardContent class="flex flex-col gap-5">
						<div>
							<Label for="username">Email</Label>
							<Input />
						</div>
						<div>
							<Label for="username">Password</Label>
							<Input />
						</div>
					</CardContent>
					<CardFooter>
						<Button type="submit" class="w-full">
							Login
						</Button>
					</CardFooter>
				</form>
			</Card>
		</main>
	);
};

export default EmployeeManagementLoginPage;
