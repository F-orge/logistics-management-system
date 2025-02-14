import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import DashboardHeader from "./_header/component";
import SidebarComponent from "./_sidebar/component";

export default async function Layout(
  { children }: {
    children: React.ReactNode;
  },
) {
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
