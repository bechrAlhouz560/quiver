import { WorkspaceCommand } from "@/commands/workspace";
import { Button } from "@/components/ui/button";
import { NewWorkspace } from "@/components/workspace/new-workspace";
import WorkspaceCard from "@/components/workspace/workspace-card";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { InfoIcon } from "lucide-react";
import { useEffect } from "react";
import { toast } from "sonner";

export function SonnerDemo() {
  return (
    <Button
      variant="outline"
      onClick={() =>
        toast("Event has been created", {
          description: "Sunday, December 03, 2023 at 9:00 AM",
          action: {
            label: "Undo",
            onClick: () => console.log("Undo"),
          },
        })
      }
    >
      Show Toast
    </Button>
  );
}
export const Route = createFileRoute("/workspaces/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: workspaces } = useQuery({
    queryFn: WorkspaceCommand.getWorkspaces,
    queryKey: ["workspaces"],
  });
  useEffect(function () {
    invoke("init_database")
      .then(function () {
        console.log("success !");
      })
      .catch((err) => {
        console.log("err ", err);
      });
  }, []);

  useEffect(
    function () {
      console.log("workspaces  = ", workspaces);
      if (workspaces) {
        console.log("workspaces", workspaces);
      }
    },
    [workspaces],
  );

  return (
    <div className=" w-screen h-screen flex items-center justify-center">
      <div className="flex flex-col gap-5 items-center px-20">
        <h1 className="text-2xl font-bold">Select a Workspace</h1>
        <div className="grid grid-cols-4 gap-4">
          <NewWorkspace />
          {workspaces?.map((workspace, key) => {
            return <WorkspaceCard workspace={workspace} key={key} />;
          })}
        </div>
      </div>
      <SonnerDemo />
    </div>
  );
}
