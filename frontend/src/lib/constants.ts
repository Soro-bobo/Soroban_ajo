export const API_URL =
  process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080/api/v1";

export const HORIZON_URL =
  process.env.NEXT_PUBLIC_HORIZON_URL ??
  "https://horizon-testnet.stellar.org";

export const NETWORK_PASSPHRASE =
  process.env.NEXT_PUBLIC_NETWORK_PASSPHRASE ??
  "Test SDF Network ; September 2015";

export const CONTRACT_ID =
  process.env.NEXT_PUBLIC_CONTRACT_ID ?? "";

export const XLM_DECIMALS = 7;
export const STROOPS_PER_XLM = 10_000_000;

export const DEFAULT_PAGE_SIZE = 20;

export const GROUP_STATUS_LABELS: Record<string, string> = {
  PENDING: "Pending",
  ACTIVE: "Active",
  COMPLETED: "Completed",
  PAUSED: "Paused",
};

export const FREQUENCY_LABELS: Record<string, string> = {
  weekly: "Weekly",
  biweekly: "Bi-weekly",
  monthly: "Monthly",
};

export const CONTRIBUTION_STATUS_LABELS: Record<string, string> = {
  pending: "Pending",
  confirmed: "Confirmed",
  failed: "Failed",
  missed: "Missed",
};

export const ROUTES = {
  HOME: "/",
  LOGIN: "/login",
  REGISTER: "/register",
  DASHBOARD: "/dashboard",
  GROUPS: "/groups",
  GROUP_DETAIL: (id: string) => `/groups/${id}`,
  GROUP_NEW: "/groups/new",
  WALLET: "/wallet",
} as const;
