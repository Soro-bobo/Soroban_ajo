"use client";

import { useCallback, useEffect } from "react";
import { useWalletStore } from "@/store/walletStore";
import {
  connectFreighter,
  fetchXlmBalance,
  getConnectedPublicKey,
} from "@/services/stellar";
import { showToast } from "./useToast";

export function useWallet() {
  const store = useWalletStore();

  const connect = useCallback(async () => {
    store.setConnecting(true);
    try {
      const info = await connectFreighter();
      store.setConnected(info.publicKey, info.network);

      const balance = await fetchXlmBalance(info.publicKey);
      store.setBalance(balance);
      showToast("Wallet connected!", "success");
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Failed to connect wallet";
      store.setError(message);
      showToast(message, "error");
    }
  }, [store]);

  const refreshBalance = useCallback(async () => {
    if (!store.address) return;
    try {
      const balance = await fetchXlmBalance(store.address);
      store.setBalance(balance);
    } catch {
      // non-critical — don't toast
    }
  }, [store]);

  // Restore session: check if freighter still connected
  useEffect(() => {
    if (store.isConnected && store.address) {
      getConnectedPublicKey().then((pk) => {
        if (pk !== store.address) {
          store.disconnect();
        } else {
          refreshBalance();
        }
      });
    }
  }, []); // intentionally runs once on mount

  return {
    address: store.address,
    balance: store.balance,
    network: store.network,
    isConnected: store.isConnected,
    isConnecting: store.isConnecting,
    error: store.error,
    connect,
    disconnect: store.disconnect,
    refreshBalance,
  };
}
