export type Environment = "development" | "staging" | "production";

export interface WorkspaceInput {
  name: string;
  description?: string;
  created_at: string;
}
export interface Workspace extends WorkspaceInput {
  id: number;
}
