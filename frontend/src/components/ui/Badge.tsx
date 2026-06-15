import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import type { GroupStatus, ContributionStatus, MemberStatus } from "@/types";

type BadgeVariant =
  | "success"
  | "warning"
  | "error"
  | "info"
  | "neutral"
  | "purple";

interface BadgeProps {
  children: React.ReactNode;
  variant?: BadgeVariant;
  className?: string;
  dot?: boolean;
}

const variantClasses: Record<BadgeVariant, string> = {
  success: "bg-green-100 text-green-800",
  warning: "bg-yellow-100 text-yellow-800",
  error: "bg-red-100 text-red-800",
  info: "bg-blue-100 text-blue-800",
  neutral: "bg-gray-100 text-gray-700",
  purple: "bg-purple-100 text-purple-800",
};

const dotColors: Record<BadgeVariant, string> = {
  success: "bg-green-500",
  warning: "bg-yellow-500",
  error: "bg-red-500",
  info: "bg-blue-500",
  neutral: "bg-gray-400",
  purple: "bg-purple-500",
};

export function Badge({ children, variant = "neutral", className, dot }: BadgeProps) {
  return (
    <span
      className={twMerge(
        clsx(
          "inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium",
          variantClasses[variant],
          className
        )
      )}
    >
      {dot && (
        <span
          className={clsx("h-1.5 w-1.5 rounded-full", dotColors[variant])}
          aria-hidden
        />
      )}
      {children}
    </span>
  );
}

export function GroupStatusBadge({ status }: { status: GroupStatus }) {
  const map: Record<GroupStatus, { variant: BadgeVariant; label: string }> = {
    PENDING: { variant: "warning", label: "Pending" },
    ACTIVE: { variant: "success", label: "Active" },
    COMPLETED: { variant: "info", label: "Completed" },
    PAUSED: { variant: "neutral", label: "Paused" },
  };
  const { variant, label } = map[status] ?? { variant: "neutral", label: status };
  return <Badge variant={variant} dot>{label}</Badge>;
}

export function ContributionStatusBadge({ status }: { status: ContributionStatus }) {
  const map: Record<ContributionStatus, { variant: BadgeVariant; label: string }> = {
    pending: { variant: "warning", label: "Pending" },
    confirmed: { variant: "success", label: "Confirmed" },
    failed: { variant: "error", label: "Failed" },
    missed: { variant: "error", label: "Missed" },
  };
  const { variant, label } = map[status] ?? { variant: "neutral", label: status };
  return <Badge variant={variant}>{label}</Badge>;
}

export function MemberStatusBadge({ status }: { status: MemberStatus }) {
  const map: Record<MemberStatus, { variant: BadgeVariant; label: string }> = {
    pending: { variant: "warning", label: "Pending" },
    active: { variant: "success", label: "Active" },
    removed: { variant: "error", label: "Removed" },
  };
  const { variant, label } = map[status] ?? { variant: "neutral", label: status };
  return <Badge variant={variant}>{label}</Badge>;
}
