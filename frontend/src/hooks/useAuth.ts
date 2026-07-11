"use client";

import { useMutation } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { api } from "@/services/api";
import { useAuthStore } from "@/store/authStore";
import type {
  ForgotPasswordPayload,
  LoginPayload,
  RegisterPayload,
  ResendVerificationPayload,
  ResetPasswordPayload,
  TokenPair,
  User,
  VerifyEmailPayload,
} from "@/types";
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
      return payload.email;
    },
    onSuccess: (email) => {
      router.push(`${ROUTES.CHECK_EMAIL}?email=${encodeURIComponent(email)}`);
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

function authErrorMessage(error: unknown, fallback: string): string {
  const message = (
    error as { response?: { data?: { error?: { message?: string } } } }
  )?.response?.data?.error?.message;
  return message ?? fallback;
}

export function useVerifyEmail() {
  return useMutation({
    mutationFn: async (payload: VerifyEmailPayload) => {
      await api.post("/auth/verify-email", payload);
    },
  });
}

export function useResendVerification() {
  return useMutation({
    mutationFn: async (payload: ResendVerificationPayload) => {
      await api.post("/auth/resend-verification", payload);
    },
    onSuccess: () => {
      showToast("If that email needs verification, a new link has been sent", "success");
    },
    onError: (error) => {
      showToast(authErrorMessage(error, "Couldn't resend verification email"), "error");
    },
  });
}

export function useForgotPassword() {
  return useMutation({
    mutationFn: async (payload: ForgotPasswordPayload) => {
      await api.post("/auth/forgot-password", payload);
    },
    onSuccess: () => {
      showToast("If that email is registered, a reset link has been sent", "success");
    },
    onError: (error) => {
      showToast(authErrorMessage(error, "Something went wrong"), "error");
    },
  });
}

export function useResetPassword() {
  const router = useRouter();

  return useMutation({
    mutationFn: async (payload: ResetPasswordPayload) => {
      await api.post("/auth/reset-password", payload);
    },
    onSuccess: () => {
      showToast("Password reset — please sign in", "success");
      router.push(ROUTES.LOGIN);
    },
    onError: (error) => {
      showToast(authErrorMessage(error, "Couldn't reset password"), "error");
    },
  });
}
