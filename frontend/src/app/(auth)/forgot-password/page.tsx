"use client";

import Link from "next/link";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Input } from "@/components/ui/Input";
import { Button } from "@/components/ui/Button";
import { useForgotPassword } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

const schema = z.object({
  email: z.string().email("Enter a valid email"),
});

type FormData = z.infer<typeof schema>;

export default function ForgotPasswordPage() {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitSuccessful },
  } = useForm<FormData>({ resolver: zodResolver(schema) });

  const { mutate: forgotPassword, isPending } = useForgotPassword();

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-50 text-center mb-1">Reset your password</h1>
      <p className="text-sm text-gray-500 dark:text-gray-400 text-center mb-6">
        Enter the email on your account and we&apos;ll send a reset link
      </p>

      <form
        onSubmit={handleSubmit((data) => forgotPassword(data))}
        className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 space-y-4 dark:bg-gray-900 dark:border-gray-800"
      >
        <Input
          label="Email"
          type="email"
          autoComplete="email"
          placeholder="you@example.com"
          error={errors.email?.message}
          {...register("email")}
        />

        <Button type="submit" className="w-full" isLoading={isPending} disabled={isSubmitSuccessful}>
          {isSubmitSuccessful ? "Link sent" : "Send reset link"}
        </Button>
      </form>

      <p className="mt-5 text-center text-sm text-gray-500 dark:text-gray-400">
        <Link href={ROUTES.LOGIN} className="text-emerald-600 hover:underline font-medium">
          Back to sign in
        </Link>
      </p>
    </div>
  );
}
