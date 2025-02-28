"use client";

import {
  Boxes,
  ChartBar,
  Contact,
  Files,
  LucideIcon,
  SquareChartGantt,
  SquareTerminal,
  Users,
} from "lucide-react";

export type Sidebar = {
  header: SidebarHeader[];
  navigation: SidebarNavigation[];
};

export type SidebarHeader = {
  name: string;
  icon: LucideIcon;
  description: string;
  href: string;
};

export type SidebarNavigation = {
  url: string;
  icon?: LucideIcon;
  isActive: boolean;
  items: {
    title: string;
    url: string;
    icon?: LucideIcon;
  }[];
};

export const SIDEBAR_ROUTING: Sidebar = {
  header: [
    {
      name: "Human Resource",
      icon: Users,
      description: "manage employees",
      href: "/management/human-resource",
    },
    {
      name: "Inventory",
      icon: Boxes,
      description: "manage employees",
      href: "/management/inventory",
    },
    {
      name: "Files",
      icon: Files,
      description: "File management",
      href: "/management/files",
    },
  ],
  navigation: [
    {
      url: "/management/human-resource",
      icon: SquareTerminal,
      isActive: true,
      items: [
        {
          title: "Overview",
          url: "/management/dashboard/human-resource/overview",
          icon: ChartBar,
        },
        {
          title: "Employees",
          url: "/management/dashboard/human-resource/employees",
          icon: Contact,
        },
        {
          title: "Teams",
          url: "/management/dashboard/human-resource/teams",
          icon: Users,
        },
        {
          title: "Tasks",
          url: "/management/dashboard/human-resource/tasks",
          icon: SquareChartGantt,
        },
      ],
    },
    {
      url: "/management/files",
      icon: SquareTerminal,
      isActive: true,
      items: [
        {
          title: "Overview",
          url: "/management/files/",
          icon: ChartBar,
        },
        {
          title: "Your files",
          url: "/management/files/owned",
          icon: Contact,
        },
        {
          title: "Shared files",
          url: "/management/files/shared",
          icon: Users,
        },
      ],
    },
  ],
};
