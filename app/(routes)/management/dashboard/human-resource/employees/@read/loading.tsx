import { Skeleton } from "@/components/ui/skeleton";
import { Fragment } from "react";

export default function Loading() {
  return (
    <div className="space-y-2.5 h-full">
      <div className="grid grid-cols-3 gap-2.5">
        <Skeleton className="h-12 w-full rounded-md" />
        <Skeleton className="h-12 w-full rounded-md" />
        <Skeleton className="h-12 w-full rounded-md" />
      </div>
      <div className="grid grid-cols-1 gap-2.5">
        {Array.from(Array(15)).map((key) => (
          <Fragment key={key}>
            <Skeleton className="h-12 w-full rounded-md" />
          </Fragment>
        ))}
      </div>
    </div>
  );
}
