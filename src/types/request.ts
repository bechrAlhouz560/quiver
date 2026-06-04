export interface RequestTab {
  id: string;
  collectionId: string | null;
  name: string;
  method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD" | "OPTIONS";
  url: string;
  headers: { key: string; value: string; enabled: boolean }[];
  params: { key: string; value: string; enabled: boolean }[];
  body: {
    type: "none" | "json" | "form" | "raw";
    content: string;
  };
  auth: {
    type: "none" | "bearer" | "basic" | "vault";
    vaultEntryId: string | null; // ref to vault, never raw value
    raw: string | null; // only for non-vault auth
  };
}

export interface RequestResponse {
  tabId: string;
  status: number;
  statusText: string;
  headers: Record<string, string>;
  body: string;
  durationMs: number;
  size: number;
  timestamp: string;
}
