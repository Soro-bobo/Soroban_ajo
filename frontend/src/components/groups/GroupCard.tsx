import Link from "next/link";
import { Users, Calendar, Coins } from "lucide-react";
import { format } from "date-fns";
import { GroupStatusBadge } from "@/components/ui/Badge";
import type { Group } from "@/types";
import { FREQUENCY_LABELS, ROUTES } from "@/lib/constants";

interface GroupCardProps {
  group: Group;
}

export function GroupCard({ group }: GroupCardProps) {
  const memberProgress = Math.round(
    (group.current_members / group.max_members) * 100
  );

  return (
    <Link
      href={ROUTES.GROUP_DETAIL(group.id)}
      className="block rounded-xl border border-gray-200 bg-white p-5 hover:border-brand-300 hover:shadow-md transition-all duration-200"
    >
      <div className="flex items-start justify-between gap-3 mb-3">
        <h3 className="font-semibold text-gray-900 text-base leading-tight line-clamp-1">
          {group.name}
        </h3>
        <GroupStatusBadge status={group.status} />
      </div>

      {group.description && (
        <p className="text-sm text-gray-500 line-clamp-2 mb-3">
          {group.description}
        </p>
      )}

      <div className="flex flex-col gap-2 text-sm text-gray-600">
        <div className="flex items-center gap-2">
          <Coins className="h-4 w-4 text-brand-600 shrink-0" />
          <span>
            <span className="font-medium text-gray-900">
              {Number(group.contribution_amount).toLocaleString()} XLM
            </span>{" "}
            / {FREQUENCY_LABELS[group.frequency] ?? group.frequency}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <Calendar className="h-4 w-4 text-gray-400 shrink-0" />
          <span>Starts {format(new Date(group.start_date), "MMM d, yyyy")}</span>
        </div>

        <div className="flex items-center gap-2">
          <Users className="h-4 w-4 text-gray-400 shrink-0" />
          <span>
            {group.current_members} / {group.max_members} members
          </span>
        </div>
      </div>

      <div className="mt-4">
        <div className="flex justify-between text-xs text-gray-500 mb-1">
          <span>Capacity</span>
          <span>{memberProgress}%</span>
        </div>
        <div className="h-1.5 bg-gray-100 rounded-full overflow-hidden">
          <div
            className="h-full bg-brand-500 rounded-full transition-all"
            style={{ width: `${memberProgress}%` }}
          />
        </div>
      </div>
    </Link>
  );
}
