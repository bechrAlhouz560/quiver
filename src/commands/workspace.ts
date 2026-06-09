import { Workspace, WorkspaceInput } from "@/types/workspace";
import { invoke } from "@tauri-apps/api/core";

// workspace commands
export abstract class WorkspaceCommand {
  static async getActiveWorkspace(): Promise<string | undefined> {
    const command = "get_active_workspace";
    const result: string | undefined = await invoke(command);
    return result;
  }

  static async setActiveWorkSpace(workspaceId: string) {
    const command = "set_active_workspace";
    const result: string | undefined = await invoke(command, {
      workspace_id: workspaceId,
    });
    return result;
  }

  static async getWorkspaces() {
    const command = "get_workspaces";
    const result: Workspace[] = await invoke(command);
    return result;
  }

  static async createWorkspace(workspace: WorkspaceInput) {
    const command = "create_workspace";
    const result: Workspace = await invoke(command, { workspace });
    return result;
  }
}
