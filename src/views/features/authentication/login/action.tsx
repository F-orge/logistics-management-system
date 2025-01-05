import { action, redirect } from "@solidjs/router";
import { z } from "zod";
import { showToast } from "~/components/ui/toast";
import { authService } from "~/entry-client";

export const authSchema = z.object({
	email: z.string().email(),
	password: z.string().min(8).nonempty(),
});

export const validateForm = (formData: FormData) => {
	const email = formData.get("email") as string;
	const password = formData.get("password") as string;
	try {
		return authSchema.parse({ email, password });
	} catch (error) {
		showToast({
			title: "Invalid form data",
			description: "Please check the form fields and try again",
		});
	}
};

export const saveToken = (token: string) => {
	localStorage.setItem("Authorization", btoa(`Bearer ${token}`));
};

export const retrieveToken = () => {
	return localStorage.getItem("Authorization");
};

export const loginAction = action(async (formData: FormData) => {
	const validatedData = validateForm(formData);

	if (!validatedData) return;

	try {
		const rpcCall = await authService.login(
			{
				email: validatedData.email,
				password: validatedData.password,
			},
			{},
		);

		saveToken(rpcCall.response.token);

		return redirect("/");
	} catch (error) {
		showToast({
			title: "Login failed",
			description: "Please check your credentials and try again",
		});

		return;
	}
});
