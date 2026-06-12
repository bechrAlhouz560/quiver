import { Workspace } from "@/types/workspace";
import { Card, CardContent, CardDescription, CardTitle } from "../ui/card";
import { LayoutDashboardIcon } from "lucide-react";
import { WorkspaceCommand } from "@/commands/workspace";
import { useRouter } from "@tanstack/react-router";
import { formatDateShort } from "@/lib/date";
export default function WorkspaceCard({ workspace }: { workspace: Workspace }) {
  const router = useRouter();
  async function setWorkspace(workspace: Workspace) {
    await WorkspaceCommand.setActiveWorkSpace(workspace.id.toString());
    router.navigate({
      href: `/dashboard`,
    });
  }
  return (
    <Card
      className="cursor-pointer hover:opacity-55 transition-opacity min-w-52 min-h-52"
      onClick={() => setWorkspace(workspace)}
    >
      <CardContent className="flex flex-col gap-2 h-full">
        <CardTitle className="text-lg font-bold flex gap-2 items-center">
          <LayoutDashboardIcon className="text-primary" /> {workspace.name}
        </CardTitle>
        <CardDescription className="flex-1">
          {workspace.description}
        </CardDescription>

        <p className="mt-auto text-xs opacity-30 ">
          {formatDateShort(workspace.created_at)}
        </p>
      </CardContent>
    </Card>
  );
}
