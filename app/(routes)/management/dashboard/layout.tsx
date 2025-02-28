import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import DashboardHeader from "./_header/component";
import SidebarComponent from "./_sidebar/component";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export default async function Layout(
  { children }: {
    children: React.ReactNode;
  },
) {
  let auth_token = (await cookies()).get("authorization");

  if (auth_token === undefined) {
    redirect("/management/login");
  }

  // check if auth_token is still valid

  return (
    <SidebarProvider>
      <SidebarComponent />
      <SidebarInset>
        <DashboardHeader />
        <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
          {children}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
