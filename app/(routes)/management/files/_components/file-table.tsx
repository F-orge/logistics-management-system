"use client";

import { updateFileVisibility } from "@/actions/files";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data-table";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useNotification } from "@/hooks/use-notification";
import { FileMetadata } from "@/lib/proto/storage";
import { formatBytes } from "@/lib/utils";
import { ColumnDef } from "@tanstack/react-table";
import { EllipsisVertical } from "lucide-react";
import { useActionState } from "react";

const columns: ColumnDef<FileMetadata>[] = [
  {
    accessorKey: "name",
    header: "Name",
    cell: ({ row }) => {
      return <Badge variant={"secondary"}>{row.getValue("name")}</Badge>;
    },
  },
  {
    accessorKey: "id",
    header: "Action",
    cell: function CellComponent({ row }) {
      return (
        <Dialog>
          <DialogTrigger asChild>
            <Button size={"icon"} variant={"ghost"}>
              <EllipsisVertical size={16} />
            </Button>
          </DialogTrigger>
          <DialogContent className="max-w-2xl">
            <DialogHeader>
              <DialogTitle>{row.original.name}</DialogTitle>
            </DialogHeader>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Type</TableHead>
                  <TableHead>Size</TableHead>
                  <TableHead>Owner Id</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                <TableRow>
                  <TableCell>
                    <Badge variant={"outline"}>{row.original.type}</Badge>
                  </TableCell>
                  <TableCell>
                    <Badge variant={"outline"}>
                      {formatBytes(row.original.size)}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <Badge variant={"outline"}>
                      {row.original.ownerId}
                    </Badge>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
            <DialogFooter>
              <Button>Download file</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      );
    },
  },
];

export function FileTable({ data }: { data: FileMetadata[] }) {
  return <DataTable columns={columns} data={data} />;
}
