"use client";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { usePathname } from "next/navigation";
import { Fragment } from "react";

export default function BreadcrumbNav() {
  const pathName = usePathname();
  return (
    <Breadcrumb>
      <BreadcrumbList>
        {pathName.split("/").slice(1).map((path, index) => (
          <Fragment key={index}>
            <BreadcrumbItem className="hidden md:block">
              <BreadcrumbLink href="#">
                {path}
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator className="last:hidden md:block" />
          </Fragment>
        ))}
      </BreadcrumbList>
    </Breadcrumb>
  );
}
