import { create } from "zustand";
import { persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

interface WalletState {
  address: string | null;
  balance: number | null;
  network: string | null;
  isConnected: boolean;
  isConnecting: boolean;
  error: string | null;
}

interface WalletActions {
  setConnected: (address: string, network: string) => void;
  setBalance: (balance: number) => void;
  setConnecting: (connecting: boolean) => void;
  setError: (error: string | null) => void;
  disconnect: () => void;
}

export const useWalletStore = create<WalletState & WalletActions>()(
  persist(
    immer((set) => ({
      address: null,
      balance: null,
      network: null,
      isConnected: false,
      isConnecting: false,
      error: null,

      setConnected: (address, network) => {
        set((state) => {
          state.address = address;
          state.network = network;
          state.isConnected = true;
          state.isConnecting = false;
          state.error = null;
        });
      },

      setBalance: (balance) => {
        set((state) => {
          state.balance = balance;
        });
      },

      setConnecting: (connecting) => {
        set((state) => {
          state.isConnecting = connecting;
        });
      },

      setError: (error) => {
        set((state) => {
          state.error = error;
          state.isConnecting = false;
        });
      },

      disconnect: () => {
        set((state) => {
          state.address = null;
          state.balance = null;
          state.network = null;
          state.isConnected = false;
          state.isConnecting = false;
          state.error = null;
        });
      },
    })),
    {
      name: "ajo-wallet",
      partialize: (state) => ({
        address: state.address,
        network: state.network,
        isConnected: state.isConnected,
      }),
    }
  )
);
