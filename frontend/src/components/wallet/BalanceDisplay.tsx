"use client";

import { RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { Skeleton } from "@/components/ui/Skeleton";
import { useWallet } from "@/hooks/useWallet";

export function BalanceDisplay() {
  const { balance, isConnected, address, refreshBalance } = useWallet();

  if (!isConnected) {
    return (
      <div className="rounded-xl border border-dashed border-gray-300 bg-gray-50 p-6 text-center">
        <p className="text-gray-500 text-sm">Connect your wallet to view balance</p>
      </div>
    );
  }

  return (
    <div className="rounded-xl border border-gray-200 bg-white p-6">
      <div className="flex items-start justify-between">
        <div>
          <p className="text-sm text-gray-500 mb-1">XLM Balance</p>
          {balance === null ? (
            <Skeleton className="h-8 w-36" />
          ) : (
            <p className="text-3xl font-bold text-gray-900">
              {balance.toLocaleString(undefined, {
                minimumFractionDigits: 2,
                maximumFractionDigits: 7,
              })}
              <span className="text-lg text-gray-500 ml-2 font-medium">XLM</span>
            </p>
          )}
          {address && (
            <p className="text-xs text-gray-400 mt-1 font-mono">
              {address.slice(0, 12)}…{address.slice(-8)}
            </p>
          )}
        </div>

        <Button
          variant="ghost"
          size="sm"
          onClick={refreshBalance}
          aria-label="Refresh balance"
          leftIcon={<RefreshCw className="h-4 w-4" />}
        >
          Refresh
        </Button>
      </div>
    </div>
  );
}
