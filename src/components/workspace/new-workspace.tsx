import { Plus } from "lucide-react";
import { Card, CardContent } from "../ui/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
} from "../ui/dialog";
import { Label } from "../ui/label";
import { Input } from "../ui/input";
import { Textarea } from "../ui/textarea";
import { Button } from "../ui/button";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";

export function NewWorkspace() {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const { mutate } = useMutation({
    mutationFn: async (data: { name: string; description: string }) => {
      // Call your backend API to create a new workspace
      console.log("data = ", data);
    },
    onError: (error) => {
      // Handle error (e.g., show a notification)

      console.log("Error creating workspace:", error);
    },
  });
  const handleCreateWorkspace = () => {
    mutate({ name, description });
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Card className="cursor-pointer border  border-dashed border-white/20 hover:opacity-55 transition-opacity flex items-center justify-center">
          <CardContent className="flex flex-col gap-5 text-primary items-center justify-center ">
            <Plus className="w-10 h-10" />
            <span className="text-sm">New Workspace</span>
          </CardContent>
        </Card>
      </DialogTrigger>
      <DialogPortal>
        <DialogOverlay />
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Create Workspace</DialogTitle>
          </DialogHeader>
          <p className="text-sm text-muted-foreground">
            Start a new workspace to organize your projects.
          </p>

          <Label>Name</Label>
          <Input
            placeholder="My Workspace"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />

          <Label>Description</Label>
          <Textarea
            placeholder="A brief description about this workspace"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />

          <Button onClick={handleCreateWorkspace}>Create Workspace</Button>
        </DialogContent>
      </DialogPortal>
    </Dialog>
  );
}
