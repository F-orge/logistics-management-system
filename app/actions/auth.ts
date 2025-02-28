"use server";

import { authClient } from "@/lib/grpc";
import { redirect } from "next/navigation";
import { RpcError } from "@protobuf-ts/runtime-rpc";
import { cookies } from "next/headers";

export async function loginAction(
  //@ts-expect
  prevState: { message: string },
  formData: FormData,
) {
  const grpcRequest = authClient.basicLogin({
    email: formData.get("email")?.toString() || "",
    password: formData.get("password")?.toString() || "",
  });
  try {
    const response = await grpcRequest.response;
    const cookieStore = await cookies();

    cookieStore.set("authorization", `Bearer ${response.accessToken}`, {
      httpOnly: true,
      secure: true,
      expires: new Date(Date.now() + Number(response.expiresIn) * 1000),
      path: "/",
    });

    redirect("/management/dashboard");
  } catch (e) {
    console.error(e);
    if (e instanceof RpcError) {
      switch (e.code) {
        case "NOT_FOUND":
          return { message: "Invalid email or password" };
      }
    } else {
      throw e;
    }
    return { message: "Internal error" };
  }
}
