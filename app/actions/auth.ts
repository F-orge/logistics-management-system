"use server";

import { authClient } from "@/lib/grpc";
import { redirect } from "next/navigation";
import { RpcError } from "@protobuf-ts/runtime-rpc";

export async function loginAction(
  //@ts-expect 
  prevState: {message:string},
  formData: FormData,
) {
  const grpcRequest = authClient.basicLogin({
    email: formData.get("email")?.toString() || "",
    password: formData.get("password")?.toString() || "",
  });
  try {
    await grpcRequest.response;
    redirect("/management/dashboard");
  } catch (e) {
    if (e instanceof RpcError) {
      switch (e.code) {
        case "INVALID_ARGUMENT":
          return { message: e.message };
      }
    }
    return { message: "Internal error" };
  }
}
