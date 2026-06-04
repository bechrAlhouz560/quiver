export type Environment = "development" | "staging" | "production";

export interface Workspace {
  id: string;
  name: string;
  description: string;
  createdAt: string;
}
