"use client";

import { format } from "date-fns";
import { ContributionStatusBadge } from "@/components/ui/Badge";
import { TableRowSkeleton } from "@/components/ui/Skeleton";
import { Button } from "@/components/ui/Button";
import { useContributions } from "@/hooks/useContributions";

interface ContributionTableProps {
  groupId: string;
}

export function ContributionTable({ groupId }: ContributionTableProps) {
  const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading } =
    useContributions(groupId);

  const contributions = data?.pages.flatMap((p) => p.data) ?? [];

  return (
    <div className="overflow-hidden rounded-xl border border-gray-200">
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left px-4 py-3 font-medium text-gray-500">
                Date
              </th>
              <th className="text-left px-4 py-3 font-medium text-gray-500">
                Amount
              </th>
              <th className="text-left px-4 py-3 font-medium text-gray-500">
                TX Hash
              </th>
              <th className="text-left px-4 py-3 font-medium text-gray-500">
                Status
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100">
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <TableRowSkeleton key={i} cols={4} />
              ))
            ) : contributions.length === 0 ? (
              <tr>
                <td
                  colSpan={4}
                  className="px-4 py-8 text-center text-gray-400"
                >
                  No contributions yet.
                </td>
              </tr>
            ) : (
              contributions.map((c) => (
                <tr key={c.id} className="hover:bg-gray-50">
                  <td className="px-4 py-3 text-gray-600">
                    {format(new Date(c.period_date), "MMM d, yyyy")}
                  </td>
                  <td className="px-4 py-3 font-medium text-gray-900">
                    {Number(c.amount).toLocaleString()} XLM
                  </td>
                  <td className="px-4 py-3 font-mono text-xs text-gray-500">
                    {c.tx_hash.slice(0, 8)}…{c.tx_hash.slice(-6)}
                  </td>
                  <td className="px-4 py-3">
                    <ContributionStatusBadge status={c.status} />
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {hasNextPage && (
        <div className="flex justify-center p-4 border-t border-gray-100">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => fetchNextPage()}
            isLoading={isFetchingNextPage}
          >
            Load more
          </Button>
        </div>
      )}
    </div>
  );
}
