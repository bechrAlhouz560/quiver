import { useQuery } from "@tanstack/react-query";
import { createFileRoute, useRouter } from "@tanstack/react-router";
import { useEffect } from "react";
import { WorkspaceCommand } from "@/commands/workspace";
export const Route = createFileRoute("/")({
  component: HomeComponent,
});

function HomeComponent() {
  const router = useRouter();

  const { data: active_workspace } = useQuery({
    queryFn: WorkspaceCommand.getActiveWorkspace,
    queryKey: ["active_workspace"],
  });
  useEffect(() => {
    if (active_workspace !== undefined && active_workspace !== "") {
      router.navigate({
        href: `/dashboard`,
      });
    } else {
      router.navigate({
        href: `/workspaces`,
      });
    }
  }, [active_workspace]);
  return (
    <div className="w-screen  h-screen flex justify-center items-center ">
      <h1>Quiver</h1>
    </div>
  );
}
