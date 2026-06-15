import {
  isConnected,
  isAllowed,
  getUserInfo,
  requestAccess,
  signTransaction,
  getNetworkDetails,
} from "@stellar/freighter-api";
import { HORIZON_URL, NETWORK_PASSPHRASE } from "@/lib/constants";

export interface FreighterUserInfo {
  publicKey: string;
  network: string;
  networkUrl: string;
}

export async function checkFreighterInstalled(): Promise<boolean> {
  try {
    return await isConnected();
  } catch {
    return false;
  }
}

export async function checkFreighterAllowed(): Promise<boolean> {
  try {
    return await isAllowed();
  } catch {
    return false;
  }
}

export async function connectFreighter(): Promise<FreighterUserInfo> {
  const installed = await checkFreighterInstalled();
  if (!installed) {
    throw new Error(
      "Freighter wallet is not installed. Please install the Freighter browser extension."
    );
  }

  const publicKey = await requestAccess();
  if (!publicKey) {
    throw new Error("Freighter access denied");
  }

  const networkDetails = await getNetworkDetails();

  return {
    publicKey,
    network: networkDetails.network ?? "TESTNET",
    networkUrl: networkDetails.networkUrl ?? HORIZON_URL,
  };
}

export async function getConnectedPublicKey(): Promise<string | null> {
  try {
    const info = await getUserInfo();
    return info.publicKey ?? null;
  } catch {
    return null;
  }
}

export async function fetchXlmBalance(publicKey: string): Promise<number> {
  const response = await fetch(`${HORIZON_URL}/accounts/${publicKey}`);
  if (!response.ok) {
    if (response.status === 404) return 0;
    throw new Error(`Horizon error: ${response.status}`);
  }

  const data = await response.json();
  const nativeBalance = data.balances?.find(
    (b: { asset_type: string; balance: string }) => b.asset_type === "native"
  );
  return nativeBalance ? parseFloat(nativeBalance.balance) : 0;
}

export async function fetchTransactionHistory(
  publicKey: string,
  limit = 10
): Promise<HorizonTransaction[]> {
  const url = `${HORIZON_URL}/accounts/${publicKey}/transactions?order=desc&limit=${limit}`;
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch transactions: ${response.status}`);
  }
  const data = await response.json();
  return data._embedded?.records ?? [];
}

export async function signXdrTransaction(xdr: string): Promise<string> {
  const signedXdr = await signTransaction(xdr, {
    networkPassphrase: NETWORK_PASSPHRASE,
  });
  if (!signedXdr) {
    throw new Error("Signing failed or was rejected");
  }
  return signedXdr;
}

export interface HorizonTransaction {
  id: string;
  hash: string;
  created_at: string;
  successful: boolean;
  fee_charged: string;
  source_account: string;
  memo_type: string;
  memo: string | null;
  ledger: number;
}
