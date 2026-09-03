"use client";

import { CheckCircle, Clock, Circle } from "lucide-react";
import { format } from "date-fns";
import type { Group, PayoutScheduleRow } from "@/types";
import { clsx } from "clsx";
import { usePayoutSchedule } from "@/hooks/useGroups";
import { Skeleton } from "@/components/ui/Skeleton";

interface PayoutScheduleProps {
  group: Group;
}

export function PayoutSchedule({ group }: PayoutScheduleProps) {
  const { data: schedule, isLoading } = usePayoutSchedule(group.id);

  if (isLoading) {
    return (
      <div className="space-y-2">
        {Array.from({ length: 3 }).map((_, i) => (
          <Skeleton key={i} className="h-14 w-full" />
        ))}
      </div>
    );
  }

  if (!schedule || schedule.length === 0) {
    if (group.status === "PENDING") {
      return (
        <p className="text-sm text-gray-400 dark:text-gray-500 italic">
          Payout schedule will be generated when the group is activated.
        </p>
      );
    }
    return <p className="text-sm text-gray-400 dark:text-gray-500">No payout schedule available.</p>;
  }

  return (
    <div className="space-y-1">
      {schedule.map((row) => {
        const isPast = row.paid_at !== null;
        const isCurrent = row.payout_round === group.current_payout_position && group.status === "ACTIVE";

        return (
          <PayoutRow
            key={row.id}
            row={row}
            isPast={isPast}
            isCurrent={isCurrent}
          />
        );
      })}
    </div>
  );
}

function PayoutRow({
  row,
  isPast,
  isCurrent,
}: {
  row: PayoutScheduleRow;
  isPast: boolean;
  isCurrent: boolean;
}) {
  return (
    <div
      className={clsx(
        "flex items-center gap-4 rounded-lg px-4 py-3 transition-colors",
        isCurrent && "bg-emerald-50 border border-emerald-200 dark:bg-emerald-500/10 dark:border-emerald-500/30",
        isPast && "bg-gray-50 dark:bg-gray-900/50",
        !isPast && !isCurrent && "bg-white border border-gray-100 dark:bg-gray-900 dark:border-gray-800"
      )}
    >
      <div className="shrink-0">
        {isPast ? (
          <CheckCircle className="h-5 w-5 text-emerald-600 dark:text-emerald-400" />
        ) : isCurrent ? (
          <Clock className="h-5 w-5 text-emerald-500 dark:text-emerald-400 animate-pulse" />
        ) : (
          <Circle className="h-5 w-5 text-gray-300 dark:text-gray-700" />
        )}
      </div>

      <div className="flex-1 min-w-0">
        <p
          className={clsx(
            "text-sm font-medium truncate",
            isCurrent ? "text-emerald-800 dark:text-emerald-300" : "text-gray-800 dark:text-gray-200"
          )}
        >
          Round {row.payout_round}: {row.display_name}
        </p>
        <p className="text-xs text-gray-500 dark:text-gray-400">
          {format(new Date(row.scheduled_date), "MMMM d, yyyy")}
          {row.paid_at && (
            <span className="ml-2 text-emerald-600 dark:text-emerald-400">
              · Paid {format(new Date(row.paid_at), "MMM d")}
            </span>
          )}
        </p>
      </div>

      <div className="shrink-0 text-right">
        {row.amount && (
          <p className="text-sm font-semibold text-gray-900 dark:text-gray-50">
            {Number(row.amount).toLocaleString()} XLM
          </p>
        )}
        {isCurrent && (
          <span className="text-xs text-emerald-600 dark:text-emerald-400 font-medium">Current</span>
        )}
        {!isPast && row.reminder_sent_at && (
          <span className="text-xs text-amber-600 dark:text-amber-400 font-medium block">Reminder sent</span>
        )}
      </div>
    </div>
  );
}
