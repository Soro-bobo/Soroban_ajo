"use client";

import { useEffect, useState } from "react";
import { format } from "date-fns";
import { CheckCircle, XCircle } from "lucide-react";
import { fetchTransactionHistory, type HorizonTransaction } from "@/services/stellar";
import { useWallet } from "@/hooks/useWallet";
import { TableRowSkeleton } from "@/components/ui/Skeleton";

export function TransactionHistory() {
  const { address, isConnected } = useWallet();
  const [transactions, setTransactions] = useState<HorizonTransaction[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!isConnected || !address) return;

    setIsLoading(true);
    setError(null);

    fetchTransactionHistory(address, 10)
      .then(setTransactions)
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load"))
      .finally(() => setIsLoading(false));
  }, [address, isConnected]);

  if (!isConnected) {
    return null;
  }

  return (
    <div className="rounded-xl border border-gray-200 overflow-hidden dark:border-gray-800">
      <div className="px-5 py-4 border-b border-gray-100 dark:border-gray-800">
        <h3 className="font-semibold text-gray-900 text-sm dark:text-gray-50">Recent Transactions</h3>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-gray-50 border-b border-gray-100 dark:bg-gray-900 dark:border-gray-800">
            <tr>
              <th className="text-left px-4 py-3 font-medium text-gray-500 text-xs dark:text-gray-400">Date</th>
              <th className="text-left px-4 py-3 font-medium text-gray-500 text-xs dark:text-gray-400">Hash</th>
              <th className="text-left px-4 py-3 font-medium text-gray-500 text-xs dark:text-gray-400">Ledger</th>
              <th className="text-left px-4 py-3 font-medium text-gray-500 text-xs dark:text-gray-400">Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-50 dark:divide-gray-800">
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => <TableRowSkeleton key={i} cols={4} />)
            ) : error ? (
              <tr>
                <td colSpan={4} className="px-4 py-6 text-center text-red-500 dark:text-red-400 text-xs">
                  {error}
                </td>
              </tr>
            ) : transactions.length === 0 ? (
              <tr>
                <td colSpan={4} className="px-4 py-6 text-center text-gray-400 dark:text-gray-500 text-xs">
                  No transactions found.
                </td>
              </tr>
            ) : (
              transactions.map((tx) => (
                <tr key={tx.id} className="hover:bg-gray-50 dark:hover:bg-gray-900">
                  <td className="px-4 py-3 text-gray-600 text-xs dark:text-gray-300">
                    {format(new Date(tx.created_at), "MMM d, HH:mm")}
                  </td>
                  <td className="px-4 py-3 font-mono text-xs text-gray-500 dark:text-gray-400">
                    {tx.hash.slice(0, 8)}…{tx.hash.slice(-6)}
                  </td>
                  <td className="px-4 py-3 text-gray-600 text-xs dark:text-gray-300">{tx.ledger}</td>
                  <td className="px-4 py-3">
                    {tx.successful ? (
                      <span className="flex items-center gap-1 text-green-600 dark:text-green-400 text-xs">
                        <CheckCircle className="h-3.5 w-3.5" />
                        Success
                      </span>
                    ) : (
                      <span className="flex items-center gap-1 text-red-500 dark:text-red-400 text-xs">
                        <XCircle className="h-3.5 w-3.5" />
                        Failed
                      </span>
                    )}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
