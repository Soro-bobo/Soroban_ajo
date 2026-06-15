"use client";

import { useMutation } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { api } from "@/services/api";
import { useAuthStore } from "@/store/authStore";
import type { LoginPayload, RegisterPayload, TokenPair, User } from "@/types";
import { showToast } from "./useToast";
import { ROUTES } from "@/lib/constants";

export function useLogin() {
  const { login } = useAuthStore();
  const router = useRouter();

  return useMutation({
    mutationFn: async (payload: LoginPayload) => {
      const { data: tokens } = await api.post<TokenPair>("/auth/login", payload);
      const { data: user } = await api.get<User>("/auth/me", {
        headers: { Authorization: `Bearer ${tokens.access_token}` },
      });
      return { tokens, user };
    },
    onSuccess: ({ tokens, user }) => {
      login(user, tokens);
      showToast("Welcome back!", "success");
      router.push(ROUTES.DASHBOARD);
    },
    onError: () => {
      showToast("Invalid email or password", "error");
    },
  });
}

export function useRegister() {
  const router = useRouter();

  return useMutation({
    mutationFn: async (payload: RegisterPayload) => {
      await api.post<User>("/auth/register", payload);
    },
    onSuccess: () => {
      showToast("Account created! Please log in.", "success");
      router.push(ROUTES.LOGIN);
    },
    onError: (error: { response?: { data?: { error?: { message?: string } } } }) => {
      const message =
        error?.response?.data?.error?.message ?? "Registration failed";
      showToast(message, "error");
    },
  });
}

export function useLogout() {
  const { logout } = useAuthStore();
  const router = useRouter();

  return () => {
    logout();
    showToast("Logged out", "info");
    router.push(ROUTES.LOGIN);
  };
}
