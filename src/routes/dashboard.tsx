import { Outlet, createFileRoute } from "@tanstack/react-router";
import TopBar from "@/components/dashboard/topbar";
import SideBar from "@/components/dashboard/sidebar";

export const Route = createFileRoute("/dashboard")({
  component: DashboardLayout,
});

function DashboardLayout() {
  return (
    <div className="w-screen h-screen flex flex-col">
      <TopBar />
      <div className="flex flex-1 w-screen">
        <SideBar />
        <div className="flex-1 overflow-y-auto">
          <Outlet />{" "}
        </div>
      </div>
    </div>
  );
}
