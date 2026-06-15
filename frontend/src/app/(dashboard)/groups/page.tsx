"use client";

import Link from "next/link";
import { PlusCircle } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/Button";
import { PageHeader } from "@/components/layout/PageHeader";
import { GroupList } from "@/components/groups/GroupList";
import type { GroupStatus } from "@/types";
import { ROUTES } from "@/lib/constants";

const STATUS_TABS: Array<{ label: string; value: GroupStatus | undefined }> = [
  { label: "All", value: undefined },
  { label: "Active", value: "ACTIVE" },
  { label: "Pending", value: "PENDING" },
  { label: "Completed", value: "COMPLETED" },
];

export default function GroupsPage() {
  const [activeTab, setActiveTab] = useState<GroupStatus | undefined>(undefined);

  return (
    <div className="max-w-6xl mx-auto">
      <PageHeader
        title="Savings Groups"
        description="Browse and join community savings circles."
        action={
          <Link href={ROUTES.GROUP_NEW}>
            <Button leftIcon={<PlusCircle className="h-4 w-4" />}>
              Create Group
            </Button>
          </Link>
        }
      />

      <div className="flex gap-1 mb-6 bg-gray-100 rounded-lg p-1 w-fit">
        {STATUS_TABS.map(({ label, value }) => (
          <button
            key={label}
            onClick={() => setActiveTab(value)}
            className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
              activeTab === value
                ? "bg-white text-gray-900 shadow-sm"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            {label}
          </button>
        ))}
      </div>

      <GroupList
        status={activeTab}
        emptyMessage="No groups found for this filter."
      />
    </div>
  );
}
