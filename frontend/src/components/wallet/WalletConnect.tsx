"use client";

import { Wallet, LogOut } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { useWallet } from "@/hooks/useWallet";

export function WalletConnect() {
  const { address, isConnected, isConnecting, connect, disconnect } =
    useWallet();

  if (isConnected && address) {
    return (
      <div className="flex items-center gap-2">
        <span className="text-sm text-gray-600 font-mono">
          {address.slice(0, 6)}…{address.slice(-4)}
        </span>
        <Button
          variant="ghost"
          size="sm"
          onClick={disconnect}
          leftIcon={<LogOut className="h-4 w-4" />}
        >
          Disconnect
        </Button>
      </div>
    );
  }

  return (
    <Button
      variant="secondary"
      size="sm"
      onClick={connect}
      isLoading={isConnecting}
      leftIcon={<Wallet className="h-4 w-4" />}
    >
      Connect Wallet
    </Button>
  );
}
