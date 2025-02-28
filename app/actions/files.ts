"use server";

import { storageClient } from "@/lib/grpc";
import { revalidatePath } from "next/cache";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { RpcError } from "@protobuf-ts/runtime-rpc";

export async function uploadFileAction(
  prevState: { message: string },
  formData: FormData,
) {
  const cookieStore = await cookies();
  const auth_token = cookieStore.get("authorization");

  if (!auth_token) redirect("/login");

  const file = formData.get("file") as File | null;

  if (!file) return { message: "No file provided" };

  const grpcAction = storageClient.createFile({
    meta: {
      authorization: auth_token.value,
    },
  });

  const reader = file.stream().getReader();
  let result;
  while (!(result = await reader.read()).done) {
    await grpcAction.requests.send({
      metadata: {
        name: file.name,
        size: file.size,
        isPublic: false,
        type: file.type,
      },
      chunk: {
        chunk: result.value,
      },
    });
  }
  // finish the upload
  await grpcAction.requests.complete();

  revalidatePath("/management/files");
  return { message: "File upload complete" };
}

export async function updateFileVisibility(
  prevState: { message: string },
  formData: FormData,
) {
  const cookieStore = await cookies();
  const auth_token = cookieStore.get("authorization");

  if (!auth_token) redirect("/login");

  // true of false if formdata is empty false else if isPublic = 'on' true
  const fileId = formData.get("file-id") as string;
  const isPublic = formData.get("is-public") === "on" ? true : false;
  console.log(fileId, isPublic);
  const grpcAction = storageClient.shareFile({
    fileId,
    userIds: [],
    shareOption: {
      oneofKind: "isPublic",
      isPublic: isPublic,
    },
  }, {
    meta: {
      "authorization": auth_token.value,
    },
  });
  try {
    const response = await grpcAction.response;
    console.log(response);
  } catch (e) {
    if (e instanceof RpcError) {
      switch (e.code) {
        default:
          return { "message": "Cannot perform action" };
      }
    }
    throw e;
  }
  revalidatePath("/management/files", "page");
  return { "message": "File updated successfully" };
}
