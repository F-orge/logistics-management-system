import { A } from "@solidjs/router";
import {
	Instagram,
	Linkedin,
	Mail,
	MessageCircle,
	PhoneCall,
} from "lucide-solid";
import type { Component } from "solid-js";
import { Button } from "~/components/ui/button";
import { TextField, TextFieldInput } from "~/components/ui/text-field";

const Footer: Component<{}> = (props) => {
	return (
		<footer>
			<div class="flex flex-col desktop:flex-row justify-between p-4 gap-5">
				<div class="flex flex-col gap-2.5">
					<div class="flex flex-row gap-2.5 px-4">
						<div>
							<h4 class="heading-4">ETMAR Logistics</h4>
							<span class="muted text-xs">Going Beyond Logistics</span>
						</div>
					</div>
					<nav>
						<div class="flex flex-row gap-2.5">
							<Button variant={"ghost"} class="muted">
								Home
							</Button>
							<Button variant={"ghost"} class="muted">
								About us
							</Button>
							<Button variant={"ghost"} class="muted">
								Services
							</Button>
						</div>
					</nav>
				</div>
				<div class="grid grid-cols-1 desktop:grid-cols-3 gap-5">
					<div>
						<h4 class="large">Email</h4>
						<ul>
							<li>
								<Button
									class="muted small flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<Mail size={16} />
									etmarphils@gmail.com
								</Button>
							</li>
							<li>
								<Button
									class="muted small flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<Mail size={16} />
									sales@etmar-philippines.com
								</Button>
							</li>
							<li>
								<Button
									class="muted small flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<Mail size={16} />
									etmarcorporation@gmail.com
								</Button>
							</li>
						</ul>
					</div>
					<div>
						<h4 class="large">Phone</h4>
						<ul>
							<li>
								<Button
									class="muted small flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<PhoneCall size={16} />
									+63-915-137-9259
								</Button>
							</li>
							<li>
								<Button
									class="muted small flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<PhoneCall size={16} />
									+63-2-8523-1183
								</Button>
							</li>
						</ul>
					</div>
					<div>
						<h4 class="large">Social Media</h4>
						<ul>
							<li>
								<Button
									as={A}
									target="_blank"
									href="https://www.facebook.com/etmarphilippines"
									class="muted small inline-flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<MessageCircle size={16} />
									Facebook
								</Button>
							</li>
							<li>
								<Button
									as={A}
									href="https://www.instagram.com/etmar_philippines?igsh=a3dlbWJ5cW80NG5z"
									target="_blank"
									class="muted small inline-flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<Instagram size={16} />
									Instagram
								</Button>
							</li>
							<li>
								<Button
									as={A}
									href="https://www.linkedin.com/in/etmar-intl-logistics-corporation-7b1729178/"
									target="_blank"
									class="muted small inline-flex flex-row items-center gap-2.5 text-muted-foreground"
									variant={"link"}
								>
									<Linkedin size={16} />
									LinkedIn
								</Button>
							</li>
						</ul>
					</div>
				</div>
			</div>
			<div class="px-8 flex flex-row justify-center items-center py-4">
				<span class="muted text-xs">
					© Copyright 2025. All rights reserved.
				</span>
			</div>
		</footer>
	);
};

export default Footer;
