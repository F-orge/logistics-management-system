"use client";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { UserPlus } from "lucide-react";

export default function Page() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant={"outline"}>
          Create
          <UserPlus />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Employee</DialogTitle>
          <DialogDescription>
            Fill up information below
          </DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
