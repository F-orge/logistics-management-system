"use client";

import {
  ContainerOutlined,
  FileOutlined,
  MobileOutlined,
  TruckOutlined,
} from "@ant-design/icons";
import { Button, Typography } from "antd";

const SERVICES: {
  title: string;
  description: string;
  link: string;
  icon: React.ReactNode;
}[] = [
  {
    title: "Freight Forwarding",
    description: "Efficient international and domestic shipping solutions.",
    icon: <ContainerOutlined size={24} />,
    link: "/services",
  },
  {
    title: "Relocation",
    description: "Seamless and stress-free moving services for your home.",
    icon: <TruckOutlined size={24} />,
    link: "/services",
  },
  {
    title: "Import/Export",
    description: "Reliable import and export services for your business.",
    icon: <ContainerOutlined size={24} />,
    link: "/services",
  },
  {
    title: "Customs Clearance",
    description: "Hassle-free customs clearance for smooth logistics.",
    icon: <FileOutlined size={24} />,
    link: "/services",
  },
  {
    title: "Trucking",
    description: "Dependable trucking services for timely deliveries.",
    icon: <TruckOutlined size={24} />,
    link: "/services",
  },
  {
    title: "Door-to-Door Delivery",
    description: "Convenient delivery services right to your doorstep.",
    icon: <MobileOutlined size={24} />,
    link: "/services",
  },
];

export default function ServicesSection() {
  return (
    <section className="py-24 container mx-auto max-w-5xl">
      <Typography.Title level={2}>Services</Typography.Title>
      <div className="grid grid-cols-3 gap-5">
        {SERVICES.map((service, index) => (
          <InfoService key={index} {...service} />
        ))}
      </div>
    </section>
  );
}
function InfoService({ title, description, link, icon }: {
  title: string;
  description: string;
  link: string;
  icon: React.ReactNode;
}) {
  return (
    <div className="hover:bg-neutral-200 p-4 transition-all space-y-2.5 rounded-md border shadow-md h-full">
      <Button className="w-fit !cursor-default" type="dashed">
        {icon}
      </Button>
      <div className="flex flex-col justify-between items-start">
        <div>
          <Typography.Title level={4}>{title}</Typography.Title>
          <Typography.Paragraph>
            {description}
          </Typography.Paragraph>
        </div>
        <Button href={link} type="link" className="!px-0">Learn more</Button>
      </div>
    </div>
  );
}
