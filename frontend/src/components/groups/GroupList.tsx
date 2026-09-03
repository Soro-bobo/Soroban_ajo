"use client";

import { useRef, useCallback } from "react";
import { GroupCard } from "./GroupCard";
import { CardSkeleton } from "@/components/ui/Skeleton";
import { Button } from "@/components/ui/Button";
import { useGroups } from "@/hooks/useGroups";
import type { GroupStatus } from "@/types";

interface GroupListProps {
  status?: GroupStatus;
  emptyMessage?: string;
}

export function GroupList({
  status,
  emptyMessage = "No groups found.",
}: GroupListProps) {
  const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading, isError } =
    useGroups(status);

  const observerRef = useRef<IntersectionObserver | null>(null);
  const loadMoreRef = useCallback(
    (node: HTMLDivElement | null) => {
      if (isFetchingNextPage) return;
      if (observerRef.current) observerRef.current.disconnect();
      observerRef.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && hasNextPage) {
          fetchNextPage();
        }
      });
      if (node) observerRef.current.observe(node);
    },
    [isFetchingNextPage, hasNextPage, fetchNextPage]
  );

  if (isLoading) {
    return (
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {Array.from({ length: 6 }).map((_, i) => (
          <CardSkeleton key={i} />
        ))}
      </div>
    );
  }

  if (isError) {
    return (
      <div className="rounded-xl border border-red-200 bg-red-50 p-6 text-center dark:border-red-500/20 dark:bg-red-500/10">
        <p className="text-red-700 font-medium dark:text-red-400">Failed to load groups.</p>
        <p className="text-red-500 text-sm mt-1 dark:text-red-400/80">Please try again later.</p>
      </div>
    );
  }

  const groups = data?.pages.flatMap((p) => p.data) ?? [];

  if (groups.length === 0) {
    return (
      <div className="rounded-xl border border-dashed border-gray-300 bg-gray-50 p-12 text-center dark:border-gray-700 dark:bg-gray-900/50">
        <p className="text-gray-500 font-medium dark:text-gray-400">{emptyMessage}</p>
      </div>
    );
  }

  return (
    <div>
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {groups.map((group) => (
          <GroupCard key={group.id} group={group} />
        ))}
      </div>

      <div ref={loadMoreRef} className="mt-6 flex justify-center">
        {isFetchingNextPage && (
          <Button variant="secondary" size="sm" isLoading>
            Loading more…
          </Button>
        )}
      </div>
    </div>
  );
}
