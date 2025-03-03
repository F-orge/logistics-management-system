"use client";

import * as React from "react";
import { ChevronsUpDown } from "lucide-react";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuShortcut,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar";
import { SidebarHeader } from "./routes";
import Link from "next/link";
import { usePathname } from "next/navigation";
export function SystemSwitcher({
  systems,
}: {
  systems: SidebarHeader[];
}) {
  const pathname = usePathname();

  const { isMobile } = useSidebar();
  const [activeSystem, setActiveTeam] = React.useState(
    systems.filter((v) => pathname.startsWith(v.href))[0],
  );

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                <activeSystem.icon className="size-4" />
              </div>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-semibold">
                  {activeSystem.name}
                </span>
                <span className="truncate text-xs">
                  {activeSystem.description}
                </span>
              </div>
              <ChevronsUpDown className="ml-auto" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
            align="start"
            side={isMobile ? "bottom" : "right"}
            sideOffset={4}
          >
            <DropdownMenuLabel className="text-xs text-muted-foreground">
              Systems
            </DropdownMenuLabel>
            {systems.map((system, index) => (
              <DropdownMenuItem
                asChild
                key={system.name}
                onClick={() => setActiveTeam(system)}
                className="gap-2 p-2"
              >
                <Link href={system.href}>
                  <div className="flex size-6 items-center justify-center rounded-md border">
                    <system.icon className="size-4 shrink-0" />
                  </div>
                  {system.name}
                  <DropdownMenuShortcut>⌘{index + 1}</DropdownMenuShortcut>
                </Link>
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
