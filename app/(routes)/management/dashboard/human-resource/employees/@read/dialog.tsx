"use client";

import { AspectRatio } from "@/components/ui/aspect-ratio";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { TableCell, TableRow } from "@/components/ui/table";
import { Employee, Role } from "@/lib/proto/management";
import { ImageIcon } from "lucide-react";

export default function EmployeeInfoDialog(
  { employee }: { employee: Employee },
) {
  return (
    <Dialog>
      <DialogTrigger className="cursor-pointer" asChild>
        <TableRow>
          <TableCell className="truncate">{employee.fullName}</TableCell>
          <TableCell className="truncate">{employee.position}</TableCell>
          <TableCell className="truncate">
            {Role[employee.role]}
          </TableCell>
        </TableRow>
      </DialogTrigger>
      <DialogContent className="max-w-5xl overflow-y-auto">
        <AspectRatio
          ratio={9 / 2}
          className="bg-secondary p-1 rounded-md mt-5 flex flex-row justify-center items-center"
        >
          <ImageIcon />
        </AspectRatio>
        <DialogHeader className="flex flex-row items-center gap-2.5">
          <Avatar>
            <AvatarFallback>
              {employee.fullName.split(" ").map((name, index, arr) =>
                index === 0 || index === arr.length - 1 ? name[0] : ""
              ).join("")}
            </AvatarFallback>
          </Avatar>
          <div className="space-y-1.5">
            <DialogTitle>{employee.fullName}</DialogTitle>
            <DialogDescription>
              {Role[employee.role]} - {employee.position}
            </DialogDescription>
          </div>
        </DialogHeader>
        <div className="space-y-2.5">
          <h4 className="scroll-m-20 text-xl font-semibold tracking-tight">
            Basic information
          </h4>
          <div className="grid grid-cols-2 border rounded-md p-1 px-4 items-center">
            <div>Address</div>
            <div className="text-sm text-muted-foreground">
              {employee.address}
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
