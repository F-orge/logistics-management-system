import { A } from "@solidjs/router";
import { createEffect, createSignal, type Component } from "solid-js";
import { Button } from "~/components/ui/button";

const CookieConsent: Component<{}> = (props) => {
	const [acceptCookies, setAcceptCookies] = createSignal(false);

	createEffect(() => {
		if (acceptCookies()) {
			console.log("Cookies accepted");
			document.cookie = `cookie-accepted=true; path=/; max-age=${60 * 60 * 24 * 365}`;
			document.getElementById("toast-cl-3")?.remove();
		}
	});

	return (
		<div class="space-y-2">
			<p>By using this website, you agree to our use of cookies.</p>
			<div class="flex flex-row gap-5 items-center">
				<Button
					onClick={() => {
						setAcceptCookies(true);
					}}
					variant="default"
					size={"sm"}
				>
					Accept Cookies
				</Button>
				<A href="/privacy-policy" class="text-primary">
					Learn more
				</A>
			</div>
		</div>
	);
};

export default CookieConsent;
