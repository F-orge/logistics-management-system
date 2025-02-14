"use client";

import { Sidebar, SidebarContent, SidebarRail } from "@/components/ui/sidebar";
import SidebarNav from "./navigation";
import { SIDEBAR_ROUTING } from "./routes";

export default function SidebarComponent(
  { ...props }: React.ComponentProps<typeof Sidebar>,
) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarContent>
        <SidebarNav items={SIDEBAR_ROUTING.navigation} />
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  );
}
