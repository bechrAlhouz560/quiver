import { Box, LayoutDashboard, List, Settings, Vault } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import React, { JSX } from "react";

function SidebarButton({
  Icon,
  text,
  onClick,
}: {
  Icon: JSX.Element;
  text: string;
  onClick: () => void;
}) {
  return (
    <div
      className="py-2 px-2 flex gap-2 hover:bg-primary/20 items-center text-accent-foreground/50"
      onClick={onClick}
    >
      {Icon} <span className="text-xs">{text}</span>
    </div>
  );
}
export default function SideBar() {
  return (
    <div className="flex flex-col w-64 border-r">
      <div className="p-2 border-b">
        <Select>
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Workspace" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem value="light">Light</SelectItem>
              <SelectItem value="dark">Dark</SelectItem>
              <SelectItem value="system">System</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>

      <div className="flex flex-col">
        <div className="py-2 px-2 flex gap-2 hover:bg-primary/20 items-center text-accent-foreground/50">
          <LayoutDashboard /> <span className="text-xs">Overview</span>
        </div>
        <div className="py-2 px-2 flex gap-2 hover:bg-primary/20 items-center text-accent-foreground/50">
          <List /> <span className="text-xs">Collections</span>
        </div>
        <div className="py-2 px-2 flex gap-2 hover:bg-primary/20 items-center text-accent-foreground/50">
          <Vault /> <span className="text-xs">Vault</span>
        </div>
      </div>

      <div className="mt-auto">
        <div className="py-2 px-2 flex gap-2 hover:bg-primary/20 items-center text-accent-foreground/50">
          <Settings /> <span className="text-xs">Settings</span>
        </div>
      </div>
    </div>
  );
}
