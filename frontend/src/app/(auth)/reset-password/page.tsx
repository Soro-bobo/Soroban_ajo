"use client";

import { Suspense } from "react";
import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Input } from "@/components/ui/Input";
import { Button } from "@/components/ui/Button";
import { PasswordStrengthChecklist } from "@/components/ui/PasswordStrengthChecklist";
import { useResetPassword } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

const schema = z.object({
  new_password: z
    .string()
    .min(8, "At least 8 characters")
    .regex(/[A-Z]/, "Include at least one uppercase letter")
    .regex(/[a-z]/, "Include at least one lowercase letter")
    .regex(/[0-9]/, "Include at least one number"),
});

type FormData = z.infer<typeof schema>;

function ResetPasswordContent() {
  const searchParams = useSearchParams();
  const token = searchParams.get("token");

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
  } = useForm<FormData>({ resolver: zodResolver(schema), mode: "onChange" });

  const { mutate: resetPassword, isPending } = useResetPassword();
  const password = watch("new_password") ?? "";

  if (!token) {
    return (
      <div className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 text-center space-y-4 dark:bg-gray-900 dark:border-gray-800">
        <h1 className="text-xl font-bold text-gray-900 dark:text-gray-50">Missing reset token</h1>
        <p className="text-sm text-gray-500 dark:text-gray-400">Use the link from your password reset email.</p>
        <Link href={ROUTES.FORGOT_PASSWORD} className="block text-sm text-emerald-600 hover:underline">
          Request a new link
        </Link>
      </div>
    );
  }

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-50 text-center mb-1">Choose a new password</h1>
      <p className="text-sm text-gray-500 dark:text-gray-400 text-center mb-6">Make it something you haven&apos;t used before</p>

      <form
        onSubmit={handleSubmit((data) => resetPassword({ token, new_password: data.new_password }))}
        className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 space-y-4 dark:bg-gray-900 dark:border-gray-800"
      >
        <div className="space-y-2">
          <Input
            label="New Password"
            type="password"
            placeholder="••••••••"
            autoComplete="new-password"
            error={errors.new_password?.message}
            {...register("new_password")}
          />
          <PasswordStrengthChecklist password={password} />
        </div>

        <Button type="submit" className="w-full" isLoading={isPending} disabled={!isValid}>
          Reset password
        </Button>
      </form>
    </div>
  );
}

export default function ResetPasswordPage() {
  return (
    <Suspense fallback={null}>
      <ResetPasswordContent />
    </Suspense>
  );
}
