"use client";

import { useState, useCallback } from "react";

export type ToastVariant = "success" | "error" | "warning" | "info";

export interface Toast {
  id: string;
  message: string;
  variant: ToastVariant;
  duration?: number;
}

interface ToastState {
  toasts: Toast[];
  toast: (message: string, variant?: ToastVariant, duration?: number) => void;
  dismiss: (id: string) => void;
  success: (message: string) => void;
  error: (message: string) => void;
  warning: (message: string) => void;
  info: (message: string) => void;
}

let globalToast: ((message: string, variant?: ToastVariant, duration?: number) => void) | null = null;

export function useToast(): ToastState {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const dismiss = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const toast = useCallback(
    (message: string, variant: ToastVariant = "info", duration = 4000) => {
      const id = `toast-${Date.now()}-${Math.random().toString(36).slice(2)}`;
      const newToast: Toast = { id, message, variant, duration };

      setToasts((prev) => [...prev, newToast]);

      setTimeout(() => {
        setToasts((prev) => prev.filter((t) => t.id !== id));
      }, duration);
    },
    []
  );

  globalToast = toast;

  return {
    toasts,
    toast,
    dismiss,
    success: (msg) => toast(msg, "success"),
    error: (msg) => toast(msg, "error"),
    warning: (msg) => toast(msg, "warning"),
    info: (msg) => toast(msg, "info"),
  };
}

export function showToast(message: string, variant: ToastVariant = "info") {
  globalToast?.(message, variant);
}
