"use client";

import { Suspense, useEffect, useState } from "react";
import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { Button } from "@/components/ui/Button";
import { useVerifyEmail } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

type Status = "verifying" | "success" | "error";

function VerifyEmailContent() {
  const searchParams = useSearchParams();
  const token = searchParams.get("token");
  const { mutate: verify } = useVerifyEmail();
  const [status, setStatus] = useState<Status>(token ? "verifying" : "error");

  useEffect(() => {
    if (!token) return;
    verify(
      { token },
      {
        onSuccess: () => setStatus("success"),
        onError: () => setStatus("error"),
      }
    );
  }, [token, verify]);

  return (
    <div className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 text-center space-y-4 dark:bg-gray-900 dark:border-gray-800">
      {status === "verifying" && (
        <>
          <h1 className="text-xl font-bold text-gray-900 dark:text-gray-50">Verifying your email…</h1>
          <p className="text-sm text-gray-500 dark:text-gray-400">This will only take a moment.</p>
        </>
      )}

      {status === "success" && (
        <>
          <div className="mx-auto h-12 w-12 rounded-full bg-emerald-100 dark:bg-emerald-500/20 flex items-center justify-center">
            <span className="text-2xl">✅</span>
          </div>
          <h1 className="text-xl font-bold text-gray-900 dark:text-gray-50">Email verified</h1>
          <p className="text-sm text-gray-500 dark:text-gray-400">
            You&apos;re all set — you can now create and join savings groups.
          </p>
          <Link href={ROUTES.LOGIN}>
            <Button className="w-full">Sign in</Button>
          </Link>
        </>
      )}

      {status === "error" && (
        <>
          <div className="mx-auto h-12 w-12 rounded-full bg-red-100 dark:bg-red-500/20 flex items-center justify-center">
            <span className="text-2xl">⚠️</span>
          </div>
          <h1 className="text-xl font-bold text-gray-900 dark:text-gray-50">Verification link invalid or expired</h1>
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Request a new link from the check-your-email screen after signing in.
          </p>
          <Link href={ROUTES.LOGIN} className="block text-sm text-emerald-600 hover:underline">
            Back to sign in
          </Link>
        </>
      )}
    </div>
  );
}

export default function VerifyEmailPage() {
  return (
    <Suspense fallback={null}>
      <VerifyEmailContent />
    </Suspense>
  );
}
