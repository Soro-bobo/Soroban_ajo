"use client";

import Link from "next/link";
import { TrendingUp, Users, Wallet, ArrowRight, PlusCircle } from "lucide-react";
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
  const { data: pendingGroups } = useGroups("PENDING");

  const activeCount = activeGroups?.pages.reduce((s, p) => s + p.data.length, 0) ?? 0;
  const pendingCount = pendingGroups?.pages.reduce((s, p) => s + p.data.length, 0) ?? 0;

  const firstName = user?.display_name.split(" ")[0] ?? "there";

  return (
    <div className="max-w-6xl mx-auto">
      <PageHeader
        title={`Good day, ${firstName}!`}
        description="Here's an overview of your savings activity."
        action={
          <Link href={ROUTES.GROUP_NEW}>
            <Button leftIcon={<PlusCircle className="h-4 w-4" />}>New Group</Button>
          </Link>
        }
      />

      {/* Stats row */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-8">
        {groupsLoading ? (
          Array.from({ length: 3 }).map((_, i) => <DashboardStatSkeleton key={i} />)
        ) : (
          <>
            <StatCard
              icon={<Users className="h-5 w-5 text-emerald-600" />}
              iconBg="bg-emerald-50"
              label="Active Groups"
              value={activeCount.toString()}
              sub="currently running"
            />
            <StatCard
              icon={<TrendingUp className="h-5 w-5 text-sky-600" />}
              iconBg="bg-sky-50"
              label="Pending Groups"
              value={pendingCount.toString()}
              sub="awaiting activation"
            />
            <StatCard
              icon={<Wallet className="h-5 w-5 text-violet-600" />}
              iconBg="bg-violet-50"
              label="XLM Balance"
              value={
                isConnected && balance !== null
                  ? `${balance.toLocaleString(undefined, { maximumFractionDigits: 2 })}`
                  : "—"
              }
              sub={isConnected ? "in Freighter wallet" : "wallet not connected"}
            />
          </>
        )}
      </div>

      {/* Active groups */}
      <div className="mb-4 flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900">Active Groups</h2>
        <Link href={ROUTES.GROUPS}>
          <Button variant="ghost" size="sm" rightIcon={<ArrowRight className="h-4 w-4" />}>
            View all
          </Button>
        </Link>
      </div>
      <GroupList status="ACTIVE" emptyMessage="No active groups yet. Create or join one!" />
    </div>
  );
}

function StatCard({
  icon,
  iconBg,
  label,
  value,
  sub,
}: {
  icon: React.ReactNode;
  iconBg: string;
  label: string;
  value: string;
  sub: string;
}) {
  return (
    <div className="rounded-xl border border-gray-200 bg-white p-5">
      <div className="flex items-center gap-3 mb-3">
        <div className={`h-9 w-9 rounded-lg ${iconBg} flex items-center justify-center shrink-0`}>
          {icon}
        </div>
        <span className="text-sm text-gray-500">{label}</span>
      </div>
      <p className="text-2xl font-bold text-gray-900 tabular-nums">{value}</p>
      <p className="text-xs text-gray-400 mt-1">{sub}</p>
    </div>
  );
}
