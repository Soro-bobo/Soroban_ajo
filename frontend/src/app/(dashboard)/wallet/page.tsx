"use client";

import { PageHeader } from "@/components/layout/PageHeader";
import { BalanceDisplay } from "@/components/wallet/BalanceDisplay";
import { WalletConnect } from "@/components/wallet/WalletConnect";
import { TransactionHistory } from "@/components/wallet/TransactionHistory";
import { useWallet } from "@/hooks/useWallet";

export default function WalletPage() {
  const { isConnected } = useWallet();

  return (
    <div className="max-w-2xl mx-auto">
      <PageHeader
        title="Wallet"
        description="Connect your Freighter wallet to manage XLM contributions."
        action={!isConnected ? <WalletConnect /> : undefined}
      />

      <div className="space-y-6">
        <BalanceDisplay />
        <TransactionHistory />
      </div>
    </div>
  );
}
