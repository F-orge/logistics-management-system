"use client";
import {
  BuildOutlined,
  CustomerServiceOutlined,
  HomeOutlined,
} from "@ant-design/icons";
import type { MenuProps } from "antd";
import { Menu, Typography } from "antd";
import { useRouter } from "next/navigation";
import { useState } from "react";

type MenuItem = Required<MenuProps>["items"][number];

const items: MenuItem[] = [
  {
    label: "Home",
    key: "/",
    icon: <HomeOutlined />,
  },
  {
    label: "Services",
    key: "services",
    icon: <CustomerServiceOutlined />,
  },
  {
    label: "Company",
    key: "company",
    icon: <BuildOutlined />,
  },
];

export default function Header() {
  const router = useRouter();
  const [current, setCurrent] = useState("/");

  const onClick: MenuProps["onClick"] = (e) => {
    setCurrent(e.key);
    router.push(`/marketing/${e.key}`);
  };

  return (
    <header className="container mx-auto max-w-5xl flex flex-row items-end justify-between w-full gap-2.5 py-4">
      <div>
        <Typography.Title level={4}>ETMAR Philippines</Typography.Title>
      </div>
      <Menu
        className="!border-none"
        onClick={onClick}
        selectedKeys={[current]}
        mode="horizontal"
        items={items}
      />
    </header>
  );
}
