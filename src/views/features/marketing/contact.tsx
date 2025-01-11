import { Mail, Send, SendToBack } from "lucide-solid";
import { For, type Component } from "solid-js";
import { Button } from "~/components/ui/button";
import {
	Card,
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from "~/components/ui/card";
import {
	TextField,
	TextFieldInput,
	TextFieldLabel,
	TextFieldTextArea,
} from "~/components/ui/text-field";

const CONTACT_LIST = [
	{
		icon: <Mail />,
		name: "Etmar philippines",
		link: "etmarphils@gmail.com",
	},
	{
		icon: <Mail />,
		name: "Sales",
		link: "sales@etmar-philippines.com",
	},
	{
		icon: <Mail />,
		name: "Etmar Corporation",
		link: "etmarcorporation@gmail.com",
	},
];

const ContactSection: Component<{}> = (props) => {
	return (
		<section class="py-24 grid grid-cols-1 desktop:grid-cols-2 gap-20">
			<div class="space-y-2.5">
				<span class="text-primary">Contact us</span>
				<h2 class="heading-2">Get in touch</h2>
				<p>We are always ready to help you. Reach out to us anytime.</p>
			</div>
			<div class="grid grid-cols-1 desktop:grid-cols-3 gap-5">
				<For each={CONTACT_LIST}>
					{(contact) => (
						<Card class="shadow-lg p-4 justify-center items-start flex flex-col intersect:animate-fade-up animate-delay-300 intersect-once">
							<div class="flex flex-row gap-2.5 items-center">
								{contact.icon}
								<CardTitle>{contact.name}</CardTitle>
							</div>
							<span class="text-primary">{contact.link}</span>
						</Card>
					)}
				</For>
			</div>
			<div class="space-y-2.5">
				<span class="muted">or</span>
				<h2 class="heading-2">Message us directly</h2>
				<p>
					Draft a message and send it to us directly. We will get back to you as
					soon as possible.
				</p>
			</div>
			<div>
				<form action="" class="flex flex-col gap-5">
					<TextField>
						<TextFieldLabel>Your email</TextFieldLabel>
						<TextFieldInput type="email" />
					</TextField>
					<TextField>
						<TextFieldLabel>Your message</TextFieldLabel>
						<TextFieldTextArea />
					</TextField>
					<div class="flex flex-row gap-2.5">
						<Button variant="default">
							<Send />
							Send to Facebook
						</Button>
						<Button variant="default">
							<Send />
							Send to GMAIL
						</Button>
					</div>
				</form>
			</div>
		</section>
	);
};

export default ContactSection;
