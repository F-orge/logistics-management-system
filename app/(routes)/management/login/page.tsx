import { GalleryVertical } from "lucide-react";
import { LoginForm } from "./form";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export default async function Page() {
  // check if user is already logged in
  let auth_token = (await cookies()).get("authorization");

  if (auth_token !== undefined) {
    redirect("/management/dashboard");
  }

  return (
    <div className="grid min-h-svh lg:grid-cols-2">
      <div className="flex flex-col gap-4 p-6 md:p-10">
        <div className="flex justify-center gap-2 md:justify-start">
          <a href="#" className="flex items-center gap-2 font-medium">
            <div className="flex h-6 w-6 items-center justify-center rounded-md bg-primary text-primary-foreground">
              <GalleryVertical className="size-4" />
            </div>
            ETMAR Philippines
          </a>
        </div>
        <div className="flex flex-1 items-center justify-center">
          <div className="w-full max-w-xs">
            <LoginForm />
          </div>
        </div>
      </div>
      <div className="relative hidden bg-muted lg:block">
      </div>
    </div>
  );
}
