"use client";

import { use } from "react";
import Link from "next/link";
import { ArrowLeft, UserPlus, Play } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { GroupStatusBadge } from "@/components/ui/Badge";
import { PageHeader } from "@/components/layout/PageHeader";
import { BreadCrumb } from "@/components/ui/BreadCrumb";
import { ContributionTable } from "@/components/groups/ContributionTable";
import { PayoutSchedule } from "@/components/groups/PayoutSchedule";
import { Skeleton } from "@/components/ui/Skeleton";
import {
  useGroup,
  useGroupMembers,
  useJoinGroup,
  useActivateGroup,
  usePayoutSchedule,
} from "@/hooks/useGroups";
import { useAuthStore } from "@/store/authStore";
import { FREQUENCY_LABELS, ROUTES } from "@/lib/constants";
import { format } from "date-fns";

interface Props {
  params: Promise<{ id: string }>;
}

export default function GroupDetailPage({ params }: Props) {
  const { id } = use(params);
  const { user } = useAuthStore();
  const { data: group, isLoading: groupLoading } = useGroup(id);
  const { data: members, isLoading: membersLoading } = useGroupMembers(id);
  const { data: schedule } = usePayoutSchedule(id);
  const { mutate: joinGroup, isPending: isJoining } = useJoinGroup();
  const { mutate: activateGroup, isPending: isActivating } = useActivateGroup();

  const isMember = members?.some((m) => m.user_id === user?.id);
  const isCreator = group?.creator_id === user?.id;
  const canActivate = isCreator && group?.status === "PENDING" && (group?.current_members ?? 0) >= 2;

  const currentUserMember = members?.find((m) => m.user_id === user?.id);
  const isUpNext =
    group?.status === "ACTIVE" &&
    currentUserMember &&
    currentUserMember.payout_position === group.current_payout_position &&
    !currentUserMember.has_received_payout;
  const upNextRow = schedule?.find(
    (row) => row.payout_round === group?.current_payout_position
  );

  if (groupLoading) {
    return (
      <div className="max-w-4xl mx-auto space-y-4">
        <Skeleton className="h-5 w-64" />
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!group) {
    return (
      <div className="text-center py-16">
        <p className="text-gray-500">Group not found.</p>
        <Link href={ROUTES.GROUPS}>
          <Button variant="ghost" className="mt-4">
            Back to Groups
          </Button>
        </Link>
      </div>
    );
  }

  const actions = (
    <div className="flex items-center gap-2">
      {canActivate && (
        <Button
          variant="secondary"
          size="sm"
          onClick={() => activateGroup(id)}
          isLoading={isActivating}
          leftIcon={<Play className="h-4 w-4" />}
        >
          Activate Group
        </Button>
      )}
      {!isMember && group.status !== "COMPLETED" && (
        <Button
          onClick={() => joinGroup(id)}
          isLoading={isJoining}
          leftIcon={<UserPlus className="h-4 w-4" />}
        >
          Join Group
        </Button>
      )}
    </div>
  );

  return (
    <div className="max-w-4xl mx-auto">
      <BreadCrumb
        className="mb-4"
        items={[
          { label: "Groups", href: ROUTES.GROUPS },
          { label: group.name },
        ]}
      />

      <PageHeader title={group.name} action={actions} />

      <div className="flex flex-wrap gap-3 mb-6">
        <GroupStatusBadge status={group.status} />
        <span className="text-sm text-gray-500">
          {FREQUENCY_LABELS[group.frequency]} · {group.current_members}/
          {group.max_members} members
        </span>
        <span className="text-sm text-gray-500">
          Starts {format(new Date(group.start_date), "MMM d, yyyy")}
        </span>
      </div>

      {group.description && (
        <p className="text-gray-600 mb-6">{group.description}</p>
      )}

      {canActivate && (
        <div className="mb-6 rounded-lg bg-amber-50 border border-amber-200 px-4 py-3 text-sm text-amber-800">
          <strong>Ready to activate.</strong> This group has {group.current_members} members.
          Click &quot;Activate Group&quot; to generate the payout schedule and start contributions.
        </div>
      )}

      {isUpNext && (
        <div className="mb-6 rounded-lg bg-emerald-50 border border-emerald-200 px-4 py-3 text-sm text-emerald-800">
          <strong>You&apos;re up next!</strong> You&apos;re in line for this round&apos;s payout
          {upNextRow?.amount &&
            ` of ${Number(upNextRow.amount).toLocaleString()} XLM`}
          {upNextRow?.scheduled_date &&
            `, scheduled around ${format(new Date(upNextRow.scheduled_date), "MMM d, yyyy")}`}
          .
        </div>
      )}

      <div className="grid sm:grid-cols-3 gap-4 mb-8">
        <InfoCard
          label="Contribution"
          value={`${Number(group.contribution_amount).toLocaleString()} XLM`}
        />
        <InfoCard
          label="Total Payout"
          value={`${(Number(group.contribution_amount) * group.max_members).toLocaleString()} XLM`}
        />
        <InfoCard
          label="Payout Round"
          value={`${group.current_payout_position} / ${group.current_members}`}
        />
      </div>

      <div className="grid lg:grid-cols-2 gap-8">
        <section>
          <h2 className="text-base font-semibold text-gray-900 mb-3">
            Payout Schedule
          </h2>
          <PayoutSchedule group={group} />
        </section>

        <section>
          <h2 className="text-base font-semibold text-gray-900 mb-3">
            Members
          </h2>
          {membersLoading ? (
            <Skeleton className="h-48 w-full" />
          ) : members && members.length > 0 ? (
            <MemberList members={members} currentPosition={group.current_payout_position} />
          ) : (
            <p className="text-gray-400 text-sm">No members yet.</p>
          )}
        </section>
      </div>

      <section className="mt-8">
        <h2 className="text-base font-semibold text-gray-900 mb-3">
          Contributions
        </h2>
        <ContributionTable groupId={id} />
      </section>
    </div>
  );
}

function InfoCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-xl border border-gray-200 bg-white p-4">
      <p className="text-xs text-gray-500 mb-1">{label}</p>
      <p className="text-lg font-bold text-gray-900">{value}</p>
    </div>
  );
}

function MemberList({
  members,
  currentPosition,
}: {
  members: { id: string; display_name: string; email: string; payout_position: number; has_received_payout: boolean }[];
  currentPosition: number;
}) {
  return (
    <ul className="space-y-2">
      {members.map((m) => (
        <li
          key={m.id}
          className="flex items-center gap-3 rounded-lg border border-gray-100 bg-white px-4 py-2.5"
        >
          <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-emerald-100 text-xs font-bold text-emerald-700">
            {m.payout_position}
          </div>
          <div className="flex-1 min-w-0">
            <p className="text-sm font-medium text-gray-900 truncate">{m.display_name}</p>
            <p className="text-xs text-gray-400 truncate">{m.email}</p>
          </div>
          {m.has_received_payout && (
            <span className="text-xs font-medium text-emerald-600 bg-emerald-50 rounded-full px-2 py-0.5">
              Paid
            </span>
          )}
          {m.payout_position === currentPosition && !m.has_received_payout && (
            <span className="text-xs font-medium text-amber-600 bg-amber-50 rounded-full px-2 py-0.5">
              Next
            </span>
          )}
        </li>
      ))}
    </ul>
  );
}
