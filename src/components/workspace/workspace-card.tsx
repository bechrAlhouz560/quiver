import { Workspace } from "@/types/workspace";
import { Card, CardContent, CardDescription, CardTitle } from "../ui/card";
import { Workflow } from "lucide-react";
export default function WorkspaceCard({ workspace }: { workspace: Workspace }) {
  return (
    <Card className="cursor-pointer hover:opacity-55 transition-opacity min-w-52 min-h-52">
      <CardContent className="flex flex-col gap-2">
        <CardTitle className="text-lg font-bold flex gap-2 items-center">
          <Workflow className="text-primary" /> {workspace.name}
        </CardTitle>
        <CardDescription>{workspace.description}</CardDescription>

        <p>{workspace.created_at}</p>
      </CardContent>
    </Card>
  );
}
