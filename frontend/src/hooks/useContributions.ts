"use client";

import { useInfiniteQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/services/api";
import type {
  Contribution,
  CreateContributionPayload,
  PaginatedResponse,
} from "@/types";
import { showToast } from "./useToast";

export const CONTRIBUTION_KEYS = {
  all: ["contributions"] as const,
  forGroup: (groupId: string) =>
    [...CONTRIBUTION_KEYS.all, "group", groupId] as const,
};

export function useContributions(groupId: string) {
  return useInfiniteQuery({
    queryKey: CONTRIBUTION_KEYS.forGroup(groupId),
    queryFn: async ({ pageParam }) => {
      const params = new URLSearchParams({ limit: "20" });
      if (pageParam) params.set("cursor", pageParam as string);

      const { data } = await api.get<PaginatedResponse<Contribution>>(
        `/groups/${groupId}/contributions?${params.toString()}`
      );
      return data;
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage) => lastPage.meta.next_cursor,
    enabled: Boolean(groupId),
  });
}

export function useRecordContribution() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (payload: CreateContributionPayload) => {
      const { data } = await api.post<Contribution>("/contributions", payload);
      return data;
    },
    onSuccess: (contribution) => {
      queryClient.invalidateQueries({
        queryKey: CONTRIBUTION_KEYS.forGroup(contribution.group_id),
      });
      showToast("Contribution recorded successfully!", "success");
    },
    onError: (error: { response?: { data?: { error?: { message?: string } } } }) => {
      const message =
        error?.response?.data?.error?.message ?? "Failed to record contribution";
      showToast(message, "error");
    },
  });
}
