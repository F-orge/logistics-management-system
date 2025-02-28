"use client";

import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar";
import SidebarNav from "./navigation";
import { SIDEBAR_ROUTING } from "./routes";
import { SystemSwitcher } from "./system-switcher";

export default function SidebarComponent(
  { ...props }: React.ComponentProps<typeof Sidebar>,
) {
  return (
    <Sidebar className="bg-secondary" collapsible="icon" {...props}>
      <SidebarHeader>
        <SystemSwitcher
          systems={SIDEBAR_ROUTING.header}
        />
      </SidebarHeader>
      <SidebarContent>
        <SidebarNav items={SIDEBAR_ROUTING.navigation} />
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  );
}
