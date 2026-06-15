"use client";

import { clsx } from "clsx";
import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from "lucide-react";
import { useToast, type ToastVariant } from "@/hooks/useToast";

const icons: Record<ToastVariant, React.ReactNode> = {
  success: <CheckCircle className="h-5 w-5 text-green-500" />,
  error: <AlertCircle className="h-5 w-5 text-red-500" />,
  warning: <AlertTriangle className="h-5 w-5 text-yellow-500" />,
  info: <Info className="h-5 w-5 text-blue-500" />,
};

const variantBg: Record<ToastVariant, string> = {
  success: "border-green-200 bg-green-50",
  error: "border-red-200 bg-red-50",
  warning: "border-yellow-200 bg-yellow-50",
  info: "border-blue-200 bg-blue-50",
};

export function ToastContainer() {
  const { toasts, dismiss } = useToast();

  return (
    <div
      className="fixed bottom-4 right-4 z-50 flex flex-col gap-2 w-80"
      aria-live="polite"
      aria-label="Notifications"
    >
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={clsx(
            "flex items-start gap-3 rounded-xl border px-4 py-3 shadow-lg animate-slide-up",
            variantBg[toast.variant]
          )}
          role="alert"
        >
          <span className="shrink-0 mt-0.5">{icons[toast.variant]}</span>
          <p className="flex-1 text-sm text-gray-800">{toast.message}</p>
          <button
            onClick={() => dismiss(toast.id)}
            className="shrink-0 text-gray-400 hover:text-gray-600"
            aria-label="Dismiss"
          >
            <X className="h-4 w-4" />
          </button>
        </div>
      ))}
    </div>
  );
}
