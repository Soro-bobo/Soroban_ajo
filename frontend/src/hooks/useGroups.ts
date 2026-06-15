"use client";

import { useInfiniteQuery, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/services/api";
import type {
  CreateGroupPayload,
  Group,
  MemberWithUser,
  PaginatedResponse,
  PayoutScheduleRow,
} from "@/types";
import { showToast } from "./useToast";
import { useRouter } from "next/navigation";
import { ROUTES } from "@/lib/constants";

export const GROUP_KEYS = {
  all: ["groups"] as const,
  list: (filters?: Record<string, string>) => [...GROUP_KEYS.all, "list", filters] as const,
  detail: (id: string) => [...GROUP_KEYS.all, "detail", id] as const,
  members: (id: string) => [...GROUP_KEYS.all, "members", id] as const,
  payoutSchedule: (id: string) => [...GROUP_KEYS.all, "payout-schedule", id] as const,
};

export function useGroups(status?: string) {
  return useInfiniteQuery({
    queryKey: GROUP_KEYS.list({ status: status ?? "all" }),
    queryFn: async ({ pageParam }) => {
      const params = new URLSearchParams({ limit: "20" });
      if (status) params.set("status", status);
      if (pageParam) params.set("cursor", pageParam as string);

      const { data } = await api.get<PaginatedResponse<Group>>(
        `/groups?${params.toString()}`
      );
      return data;
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage) => lastPage.meta.next_cursor,
  });
}

export function useGroup(id: string) {
  return useQuery({
    queryKey: GROUP_KEYS.detail(id),
    queryFn: async () => {
      const { data } = await api.get<Group>(`/groups/${id}`);
      return data;
    },
    enabled: Boolean(id),
  });
}

export function useGroupMembers(groupId: string) {
  return useQuery({
    queryKey: GROUP_KEYS.members(groupId),
    queryFn: async () => {
      const { data } = await api.get<MemberWithUser[]>(
        `/groups/${groupId}/members`
      );
      return data;
    },
    enabled: Boolean(groupId),
  });
}

export function useCreateGroup() {
  const queryClient = useQueryClient();
  const router = useRouter();

  return useMutation({
    mutationFn: async (payload: CreateGroupPayload) => {
      const { data } = await api.post<Group>("/groups", payload);
      return data;
    },
    onSuccess: (group) => {
      queryClient.invalidateQueries({ queryKey: GROUP_KEYS.all });
      showToast("Group created successfully!", "success");
      router.push(ROUTES.GROUP_DETAIL(group.id));
    },
    onError: (error: { response?: { data?: { error?: { message?: string } } } }) => {
      const message =
        error?.response?.data?.error?.message ?? "Failed to create group";
      showToast(message, "error");
    },
  });
}

export function useJoinGroup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (groupId: string) => {
      const { data } = await api.post(`/groups/${groupId}/join`);
      return data;
    },
    onSuccess: (_data, groupId) => {
      queryClient.invalidateQueries({ queryKey: GROUP_KEYS.detail(groupId) });
      queryClient.invalidateQueries({ queryKey: GROUP_KEYS.members(groupId) });
      showToast("Joined group successfully!", "success");
    },
    onError: (error: { response?: { data?: { error?: { message?: string } } } }) => {
      const message =
        error?.response?.data?.error?.message ?? "Failed to join group";
      showToast(message, "error");
    },
  });
}

export function useActivateGroup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (groupId: string) => {
      const { data } = await api.patch<Group>(`/groups/${groupId}/activate`);
      return data;
    },
    onSuccess: (group) => {
      queryClient.invalidateQueries({ queryKey: GROUP_KEYS.detail(group.id) });
      queryClient.invalidateQueries({ queryKey: GROUP_KEYS.payoutSchedule(group.id) });
      queryClient.invalidateQueries({ queryKey: GROUP_KEYS.all });
      showToast("Group activated! Payout schedule generated.", "success");
    },
    onError: (error: { response?: { data?: { error?: { message?: string } } } }) => {
      const message =
        error?.response?.data?.error?.message ?? "Failed to activate group";
      showToast(message, "error");
    },
  });
}

export function usePayoutSchedule(groupId: string) {
  return useQuery({
    queryKey: GROUP_KEYS.payoutSchedule(groupId),
    queryFn: async () => {
      const { data } = await api.get<PayoutScheduleRow[]>(
        `/groups/${groupId}/payout-schedule`
      );
      return data;
    },
    enabled: Boolean(groupId),
  });
}
