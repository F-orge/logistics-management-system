"use client";

import { Button, Typography } from "antd";

export default function LandingSection() {
  return (
    <section className="grid grid-cols-2 items-center gap-4 py-24 container mx-auto max-w-5xl">
      <div>
        <Typography.Title level={1}>ETMAR Philippines</Typography.Title>
        <Typography.Paragraph>
          Hand-picked professionals and expertly crafted components, designed
          for any kind of entrepreneur.
        </Typography.Paragraph>
        <div className="flex flex-row gap-2.5">
          <Button type="primary">Contact us</Button>
          <Button type="dashed">About us</Button>
        </div>
      </div>
      <img
        className="rounded-lg"
        src={"https://images.unsplash.com/photo-1665686377065-08ba896d16fd?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=700&h=800&q=80"}
        height={800}
        width={700}
        alt="ETMAR Philippines"
      />
    </section>
  );
}
