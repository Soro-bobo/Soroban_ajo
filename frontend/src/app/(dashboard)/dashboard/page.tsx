"use client";

import Link from "next/link";
import { TrendingUp, Users, Wallet, ArrowRight } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { PageHeader } from "@/components/layout/PageHeader";
import { GroupList } from "@/components/groups/GroupList";
import { DashboardStatSkeleton } from "@/components/ui/Skeleton";
import { useAuthStore } from "@/store/authStore";
import { useWallet } from "@/hooks/useWallet";
import { useGroups } from "@/hooks/useGroups";
import { ROUTES } from "@/lib/constants";

export default function DashboardPage() {
  const { user } = useAuthStore();
  const { balance, isConnected } = useWallet();
  const { data: activeGroups, isLoading: groupsLoading } = useGroups("ACTIVE");

  const totalGroups =
    activeGroups?.pages.reduce((sum, p) => sum + p.data.length, 0) ?? 0;

  return (
    <div className="max-w-6xl mx-auto">
      <PageHeader
        title={`Welcome back${user ? `, ${user.display_name}` : ""}!`}
        description="Here's your savings overview."
        action={
          <Link href={ROUTES.GROUP_NEW}>
            <Button leftIcon={<Users className="h-4 w-4" />}>
              New Group
            </Button>
          </Link>
        }
      />

      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-8">
        {groupsLoading ? (
          Array.from({ length: 3 }).map((_, i) => <DashboardStatSkeleton key={i} />)
        ) : (
          <>
            <StatCard
              icon={<Users className="h-5 w-5 text-brand-600" />}
              label="Active Groups"
              value={totalGroups.toString()}
              sub="you're participating in"
            />
            <StatCard
              icon={<Wallet className="h-5 w-5 text-stellar-blue" />}
              label="XLM Balance"
              value={
                isConnected && balance !== null
                  ? `${balance.toLocaleString(undefined, { maximumFractionDigits: 2 })} XLM`
                  : "—"
              }
              sub={isConnected ? "in connected wallet" : "wallet not connected"}
            />
            <StatCard
              icon={<TrendingUp className="h-5 w-5 text-purple-600" />}
              label="Next Payout"
              value="—"
              sub="no upcoming payouts"
            />
          </>
        )}
      </div>

      <div className="mb-4 flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900">Active Groups</h2>
        <Link href={ROUTES.GROUPS}>
          <Button
            variant="ghost"
            size="sm"
            rightIcon={<ArrowRight className="h-4 w-4" />}
          >
            View all
          </Button>
        </Link>
      </div>

      <GroupList status="ACTIVE" emptyMessage="You haven't joined any active groups yet." />
    </div>
  );
}

function StatCard({
  icon,
  label,
  value,
  sub,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
  sub: string;
}) {
  return (
    <div className="rounded-xl border border-gray-200 bg-white p-5">
      <div className="flex items-center gap-2 mb-3">
        <div className="h-8 w-8 rounded-lg bg-gray-50 flex items-center justify-center">
          {icon}
        </div>
        <span className="text-sm text-gray-500">{label}</span>
      </div>
      <p className="text-2xl font-bold text-gray-900">{value}</p>
      <p className="text-xs text-gray-400 mt-1">{sub}</p>
    </div>
  );
}
