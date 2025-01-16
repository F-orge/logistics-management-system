import type { Action } from "@solidjs/router";
import type { Component, JSXElement } from "solid-js";
import {
	TextField,
	TextFieldInput,
	TextFieldLabel,
} from "~/components/ui/text-field";

export const LoginForm: Component<{
	action: Action<[formData: FormData], void, [formData: FormData]>;
	children?: JSXElement;
}> = (props) => {
	return (
		<form
			action={props.action}
			method="post"
			class="flex flex-col gap-5 w-full"
		>
			<TextField>
				<TextFieldLabel>Email Address</TextFieldLabel>
				<TextFieldInput type="email" name="email" required />
			</TextField>
			<TextField>
				<div class="flex flex-row items-center justify-between py-1">
					<TextFieldLabel>Password</TextFieldLabel>
					<TextFieldLabel class="cursor-pointer text-primary">
						Forgot password?
					</TextFieldLabel>
				</div>
				<TextFieldInput type="password" name="password" required />
			</TextField>
			{props.children}
		</form>
	);
};
