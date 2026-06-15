"use client";

import Link from "next/link";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Input } from "@/components/ui/Input";
import { Button } from "@/components/ui/Button";
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
    formState: { errors },
  } = useForm<FormData>({ resolver: zodResolver(schema) });

  const { mutate: doRegister, isPending } = useRegister();

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 px-4 py-12">
      <div className="w-full max-w-sm">
        <div className="text-center mb-8">
          <div className="mx-auto mb-4 h-12 w-12 rounded-xl bg-brand-600 flex items-center justify-center">
            <span className="text-white font-bold text-xl">A</span>
          </div>
          <h1 className="text-2xl font-bold text-gray-900">Create your account</h1>
          <p className="mt-1 text-sm text-gray-500">Start saving with your community</p>
        </div>

        <form
          onSubmit={handleSubmit((data) =>
            doRegister({
              ...data,
              wallet_address: data.wallet_address || undefined,
            })
          )}
          className="bg-white rounded-2xl border border-gray-200 shadow-sm p-6 space-y-4"
        >
          <Input
            label="Display Name"
            placeholder="Amara Okonkwo"
            error={errors.display_name?.message}
            {...register("display_name")}
          />

          <Input
            label="Email"
            type="email"
            placeholder="you@example.com"
            error={errors.email?.message}
            {...register("email")}
          />

          <Input
            label="Password"
            type="password"
            placeholder="••••••••"
            error={errors.password?.message}
            hint="8+ chars, uppercase, lowercase, number"
            {...register("password")}
          />

          <Input
            label="Stellar Wallet Address"
            placeholder="G..."
            error={errors.wallet_address?.message}
            hint="Optional — you can connect Freighter later"
            {...register("wallet_address")}
          />

          <Button type="submit" className="w-full" isLoading={isPending}>
            Create Account
          </Button>
        </form>

        <p className="mt-5 text-center text-sm text-gray-500">
          Already have an account?{" "}
          <Link href={ROUTES.LOGIN} className="text-brand-600 hover:underline font-medium">
            Sign in
          </Link>
        </p>
      </div>
    </div>
  );
}
