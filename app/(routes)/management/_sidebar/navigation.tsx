"use client";

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { type SidebarNavigation } from "./routes";
import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export default function Component({
  items,
}: {
  items: SidebarNavigation[];
}) {
  const pathname = usePathname();

  return (
    <SidebarGroup>
      <SidebarGroupLabel className="text-muted-foreground">
        Navigation
      </SidebarGroupLabel>
      <SidebarMenu>
        {items.map((item) => {
          if (pathname.startsWith(item.url)) {
            return (
              <React.Fragment key={item.url}>
                {item.items?.map((subItem) => (
                  <SidebarMenuItem key={subItem.url}>
                    <SidebarMenuButton
                      isActive={pathname === subItem.url}
                      asChild
                      tooltip={subItem.title}
                      key={subItem.title}
                    >
                      <Link
                        className="flex"
                        href={subItem.url}
                      >
                        {subItem.icon && <subItem.icon size={16} />}
                        <span>{subItem.title}</span>
                      </Link>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </React.Fragment>
            );
          }
          return null;
        })}
      </SidebarMenu>
    </SidebarGroup>
  );
}
