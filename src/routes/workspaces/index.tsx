import { NewWorkspace } from "@/components/workspace/new-workspace";
import WorkspaceCard from "@/components/workspace/workspace-card";
import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
invoke;
export const Route = createFileRoute("/workspaces/")({
  component: RouteComponent,
});

function RouteComponent() {
  useEffect(function () {
    invoke("init_database")
      .then(function () {
        console.log("success !");
      })
      .catch((err) => {
        console.log("err ", err);
      });
  }, []);
  return (
    <div className=" w-screen h-screen flex items-center justify-center">
      <div className="flex flex-col gap-5 items-center px-20">
        <h1 className="text-2xl font-bold">Select a Workspace</h1>
        <div className="grid grid-cols-4 gap-4">
          <NewWorkspace />
          <WorkspaceCard />
          <WorkspaceCard />
          <WorkspaceCard />
        </div>
      </div>
    </div>
  );
}
