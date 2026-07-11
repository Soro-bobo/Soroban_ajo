"use client";

import { Suspense, useState } from "react";
import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { Button } from "@/components/ui/Button";
import { useResendVerification } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

function CheckEmailContent() {
  const searchParams = useSearchParams();
  const email = searchParams.get("email") ?? "";
  const { mutate: resend, isPending } = useResendVerification();
  const [sent, setSent] = useState(false);

  return (
    <div className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 text-center space-y-4">
      <div className="mx-auto h-12 w-12 rounded-full bg-emerald-100 flex items-center justify-center">
        <span className="text-2xl">✉️</span>
      </div>
      <h1 className="text-xl font-bold text-gray-900">Check your email</h1>
      <p className="text-sm text-gray-500">
        We sent a verification link to{" "}
        {email ? <span className="font-medium text-gray-700">{email}</span> : "your email address"}.
        Click the link to activate your account.
      </p>
      <p className="text-xs text-gray-400">
        You can sign in before verifying, but you&apos;ll need to verify your email to create or join
        savings groups.
      </p>

      <Button
        variant="secondary"
        className="w-full"
        isLoading={isPending}
        disabled={!email || sent}
        onClick={() => {
          if (!email) return;
          resend({ email });
          setSent(true);
        }}
      >
        {sent ? "Link sent" : "Resend verification email"}
      </Button>

      <Link href={ROUTES.LOGIN} className="block text-sm text-emerald-600 hover:underline">
        Back to sign in
      </Link>
    </div>
  );
}

export default function CheckEmailPage() {
  return (
    <Suspense fallback={null}>
      <CheckEmailContent />
    </Suspense>
  );
}
