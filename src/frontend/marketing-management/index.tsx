import { useColorMode } from "@kobalte/core";
import { createEffect, createSignal, For, type Component } from "solid-js";
import { Button } from "../components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "../components/ui/dropdown-menu";
import Input from "../components/ui/input";
import {
	FacebookIcon,
	Instagram,
	Laptop,
	Linkedin,
	Mail,
	MessageCircle,
	Moon,
	PhoneCall,
	Sun,
} from "lucide-solid";
import { A } from "@solidjs/router";
import { showToast } from "../components/ui/toast";

const COUNTRIES = [
	"Philippines",
	"United States",
	"Canada",
	"United Kingdom",
	"Germany",
	"France",
	"Italy",
	"Spain",
	"Australia",
	"Japan",
	"China",
	"India",
	"Brazil",
	"Mexico",
	"Russia",
	"South Korea",
	"Netherlands",
	"Switzerland",
	"Sweden",
	"Norway",
	"Denmark",
	"Finland",
	"Belgium",
	"Austria",
	"Ireland",
];

const Header: Component<{}> = (props) => {
	const { setColorMode } = useColorMode();
	return (
		<header class="p-4 flex flex-row items-center gap-5 justify-between">
			<div class="flex flex-row gap-5">
				<div class="flex flex-row gap-2.5 items-center">
					<img
						src="/etmar-logo.png"
						width={80}
						height={40}
						alt="Etmar Logistics Logo"
					/>
					<div>
						<h4 class="heading-4">ETMAR Logistics</h4>
					</div>
				</div>
				<nav class="flex flex-row gap-2.5 justify-between items-center">
					<div class="flex flex-row gap-2.5">
						<Button variant={"ghost"}>Home</Button>
						<Button variant={"ghost"}>About us</Button>
						<Button variant={"ghost"}>Services</Button>
					</div>
				</nav>
			</div>
			<div class="flex flex-row gap-2.5 items-center">
				<DropdownMenu>
					<DropdownMenuTrigger
						as={Button<"button">}
						variant="ghost"
						size="sm"
						class="w-9 px-0"
					>
						<Sun class="size-6 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
						<Moon class="absolute size-6 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
						<span class="sr-only">Toggle theme</span>
					</DropdownMenuTrigger>
					<DropdownMenuContent>
						<DropdownMenuItem onSelect={() => setColorMode("light")}>
							<Sun class="mr-2 size-4" />
							<span>Light</span>
						</DropdownMenuItem>
						<DropdownMenuItem onSelect={() => setColorMode("dark")}>
							<Moon class="mr-2 size-4" />
							<span>Dark</span>
						</DropdownMenuItem>
						<DropdownMenuItem onSelect={() => setColorMode("system")}>
							<Laptop class="mr-2 size-4" />
							<span>System</span>
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
				<Button variant={"default"} size={"sm"}>
					Contact us
				</Button>
			</div>
		</header>
	);
};

const Footer: Component<{}> = (props) => {
	return (
		<footer>
			<div class="flex flex-row justify-between border-t p-4">
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
				<div class="grid grid-cols-3 gap-5">
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
			<div class="px-8 flex flex-row justify-between items-center py-4">
				<span class="muted text-xs">
					© Copyright 2025. All rights reserved.
				</span>
				<div class="w-1/2 flex flex-row gap-5">
					<Input placeholder="Email Address" />
					<Button variant={"default"}>Join our newsletter</Button>
				</div>
			</div>
		</footer>
	);
};

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

const MarketingHomePage: Component<{}> = (props) => {
	if (!window.document.cookie.includes("cookie-accepted=true")) {
		console.log(window.document.cookie);
		showToast({
			title: "Accept Cookies",
			description: <CookieConsent />,
			duration: 10000,
		});
	}

	return (
		<main class="h-screen max-h-screen container mx-auto max-w-[1920px]">
			<Header />
			<article>
				<section class="py-24 flex flex-col  justify-center gap-5">
					<h1 class="heading-1 text-primary font-extrabold">
						Going Beyond Logistics
					</h1>
					<p class="paragraph w-1/2">
						ETMAR is a global exporter specializing in premium Philippine
						products, including a wide range of food and non-food items,
						delivering quality and authenticity to international markets.
					</p>
					<ul>
						<li>
							<Button variant="default">Start Shipping</Button>
						</li>
					</ul>
				</section>
				<section class="py-12 rounded-md px-4 w-full flex flex-col gap-5 bg-foreground overflow-x-hidden text-accent relative">
					<h2 class="text-center heading-2">Ship to over 20+ countries</h2>
					<div class="w-full inline-flex flex-nowrap">
						<For each={[...new Array(8)]}>
							{(_, index) => (
								<ul class="flex items-center justify-center md:justify-start [&_li]:mx-8 [&_img]:max-w-none animate-infinite-scroll">
									<For each={COUNTRIES}>
										{(country) => <li class="small">{country}</li>}
									</For>
								</ul>
							)}
						</For>
					</div>
					<div class="absolute top-0 left-0 w-12 h-full bg-gradient-to-r from-foreground to-transparent pointer-events-none" />
					<div class="absolute top-0 right-0 w-12 h-full bg-gradient-to-l from-foreground to-transparent pointer-events-none" />
				</section>
				<section class="py-24 px-4">Services</section>
				<section class="py-24 bg-foreground text-accent rounded-md px-4">
					Location / Address
				</section>
				<section class="py-24 rounded-md px-4">Testimonials</section>
				<section class="py-24 bg-foreground text-accent px-4">
					Contact Form
				</section>
			</article>
			<Footer />
		</main>
	);
};

export default MarketingHomePage;
