import { cookies } from "next/headers";
import { FileTable } from "./_components/file-table";
import { UploadFileDialog } from "./_components/upload-dialog";
import { storageClient } from "@/lib/grpc";
import { redirect } from "next/navigation";
import { cache } from "react";

const getFiles = cache(async () => {
  const auth_token = (await cookies()).get("authorization");

  if (!auth_token) redirect("/login");

  const files = [];
  const ownedFiles = storageClient.listOwnedFiles({}, {
    meta: {
      "authorization": auth_token.value,
    },
  });
  const sharedFiles = storageClient.listSharedFiles({}, {
    meta: {
      "authorization": auth_token.value,
    },
  });

  for await (const response of ownedFiles.responses) {
    files.push(response);
  }

  for await (const response of sharedFiles.responses) {
    files.push(response);
  }

  return files;
});

export default async function Page() {
  // get cookies
  const files = await getFiles();

  return (
    <article className="space-y-5">
      <section>
        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Files
        </h2>
      </section>
      <section className="flex flex-row justify-end">
        <UploadFileDialog />
      </section>
      <section>
        <FileTable data={files} />
      </section>
    </article>
  );
}
