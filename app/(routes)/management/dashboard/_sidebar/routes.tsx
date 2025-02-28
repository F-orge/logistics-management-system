"use client";

import {
  ChartBar,
  Contact,
  SquareChartGantt,
  SquareTerminal,
  Users,
} from "lucide-react";
import React from "react";

export type Sidebar = {
  header: SidebarHeader;
  navigation: SidebarNavigation[];
};

export type SidebarHeader = {
  name: string;
};

export type SidebarNavigation = {
  title: string;
  url: string;
  icon?: React.ReactNode;
  isActive: boolean;
  items: {
    title: string;
    url: string;
    icon?: React.ReactNode;
  }[];
};

export const SIDEBAR_ROUTING: Sidebar = {
  header: {
    name: "Dashboard",
  },
  navigation: [
    {
      title: "Human Resources",
      url: "/management/human-resource",
      icon: <SquareTerminal />,
      isActive: true,
      items: [
        {
          title: "Overview",
          url: "/management/dashboard/human-resource/overview",
          icon: <ChartBar />,
        },
        {
          title: "Employees",
          url: "/management/dashboard/human-resource/employees",
          icon: <Contact />,
        },
        {
          title: "Teams",
          url: "/management/dashboard/human-resource/teams",
          icon: <Users />,
        },
        {
          title: "Tasks",
          url: "/management/dashboard/human-resource/tasks",
          icon: <SquareChartGantt />,
        },
      ],
    },
  ],
};
