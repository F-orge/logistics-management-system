"use client";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { uploadFileAction } from "@/actions/files";
import { useActionState } from "react";
import { useNotification } from "@/hooks/use-notification";

export function UploadFileDialog() {
  const [state, formAction, pending] = useActionState(uploadFileAction, {
    message: "",
  });

  useNotification(state.message);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Upload file</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Upload file</DialogTitle>
        </DialogHeader>
        <form className="space-y-8" action={formAction}>
          <Input placeholder="abc.pdf" name="file" type="file" required />
          <DialogClose asChild>
            <Button isLoading={pending} type="submit">CreateFile</Button>
          </DialogClose>
        </form>
      </DialogContent>
    </Dialog>
  );
}
