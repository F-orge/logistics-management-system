import { useColorMode } from "@kobalte/core";
import { Laptop, List, Moon, Sun } from "lucide-solid";
import type { Component } from "solid-js";
import { Button } from "~/components/ui/button";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";

const Header: Component<{}> = (props) => {
	const { setColorMode } = useColorMode();
	return (
		<header class="p-4 flex flex-row items-center gap-5 justify-between ">
			<div class="flex flex-row gap-2.5 items-center">
				<img
					src="/etmar-logo.png"
					width={80}
					height={40}
					alt="Etmar Logistics Logo"
				/>
				<div>
					<h4 class="hidden desktop:block heading-4">ETMAR Logistics</h4>
				</div>
			</div>
			<div class="hidden desktop:flex flex-row gap-2.5 items-center">
				<nav class="flex flex-row gap-2.5 justify-between items-center">
					<div class="flex flex-row gap-2.5">
						<Button variant={"ghost"}>Home</Button>
						<Button variant={"ghost"}>About us</Button>
						<Button variant={"ghost"}>Services</Button>
					</div>
				</nav>
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
				<Button variant={"outline"} size={"sm"}>
					Contact us
				</Button>
			</div>
			<div class="block desktop:hidden">
				<DropdownMenu>
					<DropdownMenuTrigger>
						<List size={16} />
					</DropdownMenuTrigger>
					<DropdownMenuContent class="max-w-full w-screen">
						<DropdownMenuLabel>Navigation</DropdownMenuLabel>
						<DropdownMenuSeparator />
						<DropdownMenuItem>Home</DropdownMenuItem>
						<DropdownMenuItem>About us</DropdownMenuItem>
						<DropdownMenuItem>Services</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>
		</header>
	);
};

export default Header;
