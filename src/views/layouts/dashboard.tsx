import { A, type RouteSectionProps } from "@solidjs/router";
import {
	File,
	Grid2X2,
	House,
	List,
	ListTodo,
	Settings,
	UsersRound,
} from "lucide-solid";
import { For, type Component } from "solid-js";
import { Avatar, AvatarFallback } from "~/components/ui/avatar";
import { Button } from "~/components/ui/button";
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarGroupContent,
	SidebarGroupLabel,
	SidebarHeader,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
	SidebarProvider,
} from "~/components/ui/sidebar";

const MANAGEMENT_NAVIGATION = [
	{
		groupName: "Management",
		items: [
			{
				icon: House,
				name: "Home",
				link: "/",
			},
			{
				icon: File,
				name: "Files",
				link: "/files",
			},
			{
				icon: UsersRound,
				name: "Employees",
				link: "/employees",
			},
			{
				icon: Grid2X2,
				name: "Department",
				link: "/department",
			},
			{
				icon: ListTodo,
				name: "Task",
				link: "/task",
			},
		],
	},
	{
		groupName: "Settings",
		items: [
			{
				icon: Settings,
				name: "Settings",
				link: "/settings",
			},
		],
	},
];

const ManagementAppSidebar: Component<{}> = (props) => {
	return (
		<Sidebar class="">
			<SidebarContent>
				<For each={MANAGEMENT_NAVIGATION}>
					{(group) => (
						<SidebarGroup>
							<SidebarGroupLabel class="lead px-0 text-primary-foreground">
								{group.groupName}
							</SidebarGroupLabel>
							<SidebarGroupContent>
								<SidebarMenu class="gap-2.5">
									<For each={group.items}>
										{(item) => (
											<SidebarMenuItem>
												<SidebarMenuButton
													as={A}
													href={item.link}
													variant={"outline"}
												>
													<item.icon class="text-primary" />
													<span class="small muted">{item.name}</span>
												</SidebarMenuButton>
											</SidebarMenuItem>
										)}
									</For>
								</SidebarMenu>
							</SidebarGroupContent>
						</SidebarGroup>
					)}
				</For>
				<SidebarGroup />
			</SidebarContent>
			<SidebarFooter class="border-t">
				<div class="flex flex-row items-center justify-between">
					<div class="flex flex-row items-center gap-2.5">
						<Avatar class="h-8 w-8">
							<AvatarFallback class="text-sm">US</AvatarFallback>
						</Avatar>
						<span class="large font-normal muted">Username</span>
					</div>
					<Button variant={"ghost"}>
						<List class="text-primary" />
					</Button>
				</div>
			</SidebarFooter>
		</Sidebar>
	);
};

const DashboardLayout = (props: RouteSectionProps) => {
	return (
		<SidebarProvider open>
			<ManagementAppSidebar />
			<main class="p-4">{props.children}</main>
		</SidebarProvider>
	);
};

export default DashboardLayout;
