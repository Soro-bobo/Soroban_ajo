// ─── Auth ────────────────────────────────────────────────────────────────────

export interface User {
  id: string;
  email: string;
  display_name: string;
  wallet_address: string | null;
  is_active: boolean;
  email_verified: boolean;
  created_at: string;
}

export interface TokenPair {
  access_token: string;
  refresh_token: string;
  expires_in: number;
}

export interface RegisterPayload {
  email: string;
  password: string;
  display_name: string;
  wallet_address?: string;
}

export interface LoginPayload {
  email: string;
  password: string;
}

export interface VerifyEmailPayload {
  token: string;
}

export interface ResendVerificationPayload {
  email: string;
}

export interface ForgotPasswordPayload {
  email: string;
}

export interface ResetPasswordPayload {
  token: string;
  new_password: string;
}

// ─── Groups ──────────────────────────────────────────────────────────────────

export type GroupStatus = "PENDING" | "ACTIVE" | "COMPLETED" | "PAUSED";
export type ContributionFrequency = "weekly" | "biweekly" | "monthly";

export interface Group {
  id: string;
  name: string;
  description: string | null;
  contribution_amount: string;
  frequency: ContributionFrequency;
  max_members: number;
  current_members: number;
  status: GroupStatus;
  start_date: string;
  creator_id: string;
  contract_group_id: string | null;
  current_payout_position: number;
  created_at: string;
  updated_at: string;
}

export interface CreateGroupPayload {
  name: string;
  description?: string;
  contribution_amount: string;
  frequency: ContributionFrequency;
  max_members: number;
  start_date: string;
}

// ─── Members ─────────────────────────────────────────────────────────────────

export type MemberStatus = "pending" | "active" | "removed";

export interface Member {
  id: string;
  group_id: string;
  user_id: string;
  payout_position: number;
  status: MemberStatus;
  has_received_payout: boolean;
  joined_at: string;
  updated_at: string;
}

export interface MemberWithUser extends Member {
  display_name: string;
  email: string;
  wallet_address: string | null;
}

// ─── Contributions ───────────────────────────────────────────────────────────

export type ContributionStatus = "pending" | "confirmed" | "failed" | "missed";

export interface Contribution {
  id: string;
  group_id: string;
  member_id: string;
  amount: string;
  tx_hash: string;
  status: ContributionStatus;
  period_date: string;
  confirmed_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateContributionPayload {
  group_id: string;
  tx_hash: string;
  amount: string;
}

// ─── Pagination ──────────────────────────────────────────────────────────────

export interface PageMeta {
  total: number;
  limit: number;
  next_cursor: string | null;
  has_more: boolean;
}

export interface PaginatedResponse<T> {
  data: T[];
  meta: PageMeta;
}

// ─── API Errors ──────────────────────────────────────────────────────────────

export interface ApiErrorBody {
  error: {
    status: number;
    message: string;
  };
}

// ─── Wallet ──────────────────────────────────────────────────────────────────

export interface WalletState {
  address: string | null;
  balance: number | null;
  isConnected: boolean;
  isConnecting: boolean;
  network: string | null;
}

// ─── Payout Schedule ─────────────────────────────────────────────────────────

export interface PayoutScheduleEntry {
  round: number;
  member: MemberWithUser;
  scheduled_date: string;
  paid_at: string | null;
  is_current: boolean;
}

export interface PayoutScheduleRow {
  id: string;
  group_id: string;
  member_id: string;
  display_name: string;
  payout_round: number;
  scheduled_date: string;
  paid_at: string | null;
  reminder_sent_at: string | null;
  amount: string | null;
}
