"use client";

import Link from "next/link";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Input } from "@/components/ui/Input";
import { Button } from "@/components/ui/Button";
import { PasswordStrengthChecklist } from "@/components/ui/PasswordStrengthChecklist";
import { useRegister } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

const schema = z.object({
  display_name: z.string().min(2, "Name must be at least 2 characters"),
  email: z.string().email("Enter a valid email"),
  password: z
    .string()
    .min(8, "At least 8 characters")
    .regex(/[A-Z]/, "Include at least one uppercase letter")
    .regex(/[a-z]/, "Include at least one lowercase letter")
    .regex(/[0-9]/, "Include at least one number"),
  wallet_address: z
    .string()
    .length(56, "Must be a valid Stellar address (56 characters)")
    .optional()
    .or(z.literal("")),
});

type FormData = z.infer<typeof schema>;

export default function RegisterPage() {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
  } = useForm<FormData>({ resolver: zodResolver(schema), mode: "onChange" });

  const { mutate: doRegister, isPending } = useRegister();
  const password = watch("password") ?? "";

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-50 text-center mb-1">Create your account</h1>
      <p className="text-sm text-gray-500 dark:text-gray-400 text-center mb-6">Start saving with your community</p>

      <form
        onSubmit={handleSubmit((data) =>
          doRegister({
            ...data,
            wallet_address: data.wallet_address || undefined,
          })
        )}
        className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 space-y-4 dark:bg-gray-900 dark:border-gray-800"
      >
        <Input
          label="Display Name"
          placeholder="Amara Okonkwo"
          autoComplete="name"
          error={errors.display_name?.message}
          {...register("display_name")}
        />

        <Input
          label="Email"
          type="email"
          placeholder="you@example.com"
          autoComplete="email"
          error={errors.email?.message}
          {...register("email")}
        />

        <div className="space-y-2">
          <Input
            label="Password"
            type="password"
            placeholder="••••••••"
            autoComplete="new-password"
            error={errors.password?.message}
            {...register("password")}
          />
          <PasswordStrengthChecklist password={password} />
        </div>

        <Input
          label="Stellar Wallet Address"
          placeholder="G..."
          error={errors.wallet_address?.message}
          hint="Optional — connect Freighter later in the Wallet tab"
          {...register("wallet_address")}
        />

        <Button type="submit" className="w-full" isLoading={isPending} disabled={!isValid}>
          Create Account
        </Button>
      </form>

      <p className="mt-5 text-center text-sm text-gray-500 dark:text-gray-400">
        Already have an account?{" "}
        <Link href={ROUTES.LOGIN} className="text-emerald-600 hover:underline font-medium">
          Sign in
        </Link>
      </p>
    </div>
  );
}
