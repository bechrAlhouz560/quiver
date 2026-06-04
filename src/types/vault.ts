import { Environment } from "./workspace";

export interface VaultEntry {
  id: string;
  workspaceId: string;
  environment: Environment;
  name: string; // e.g. "Stripe Secret Key"
  tags: string[];
  createdAt: string;
  // value is never stored in JS — lives in Rust/SQLite only
}
