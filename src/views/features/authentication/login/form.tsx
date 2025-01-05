import type { Action } from "@solidjs/router";
import type { Component, JSXElement } from "solid-js";
import {
	TextField,
	TextFieldInput,
	TextFieldLabel,
} from "~/components/ui/text-field";

export const LoginForm: Component<{
	action: Action<[formData: FormData], void, [formData: FormData]>;
	children: JSXElement;
}> = (props) => {
	return (
		<form action={props.action} method="post" class="flex flex-col gap-5">
			<TextField>
				<TextFieldLabel>Email</TextFieldLabel>
				<TextFieldInput type="email" name="email" required />
			</TextField>
			<TextField>
				<TextFieldLabel>Password</TextFieldLabel>
				<TextFieldInput type="password" name="password" required />
			</TextField>
			{props.children}
		</form>
	);
};
