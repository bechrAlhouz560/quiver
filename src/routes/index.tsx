import { useQuery } from "@tanstack/react-query";
import { createFileRoute, useRouter } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { WorkspaceCommand } from "@/commands/workspace";
import { invoke } from "@tauri-apps/api/core";
export const Route = createFileRoute("/")({
  component: HomeComponent,
});

function HomeComponent() {
  const router = useRouter();
  const [loading, setLoading] = useState(true);
  const { data: active_workspace } = useQuery({
    queryFn: WorkspaceCommand.getActiveWorkspace,
    queryKey: ["active_workspace"],
  });

  useEffect(function () {
    invoke("init_database")
      .then(function () {
        console.log("success !");
      })
      .catch((err) => {
        console.log("err ", err);
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);
  useEffect(() => {
    if (loading) return;
    if (active_workspace !== undefined && active_workspace !== "") {
      router.navigate({
        href: `/dashboard`,
      });
    } else {
      router.navigate({
        href: `/workspaces`,
      });
    }
  }, [active_workspace, loading]);
  return (
    <div className="w-screen  h-screen flex justify-center items-center ">
      <h1>Quiver</h1>
    </div>
  );
}
