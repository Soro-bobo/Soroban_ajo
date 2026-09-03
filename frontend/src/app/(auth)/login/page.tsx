"use client";

import Link from "next/link";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Input } from "@/components/ui/Input";
import { Button } from "@/components/ui/Button";
import { useLogin } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

const schema = z.object({
  email: z.string().email("Enter a valid email"),
  password: z.string().min(1, "Password is required"),
});

type FormData = z.infer<typeof schema>;

export default function LoginPage() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormData>({ resolver: zodResolver(schema) });

  const { mutate: login, isPending } = useLogin();

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-50 text-center mb-1">Welcome back</h1>
      <p className="text-sm text-gray-500 dark:text-gray-400 text-center mb-6">Sign in to your Ajo account</p>

      <form
        onSubmit={handleSubmit((data) => login(data))}
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

        <div className="space-y-1">
          <Input
            label="Password"
            type="password"
            autoComplete="current-password"
            placeholder="••••••••"
            error={errors.password?.message}
            {...register("password")}
          />
          <div className="text-right">
            <Link href={ROUTES.FORGOT_PASSWORD} className="text-xs text-emerald-600 hover:underline">
              Forgot password?
            </Link>
          </div>
        </div>

        <Button type="submit" className="w-full" isLoading={isPending}>
          Sign in
        </Button>
      </form>

      <p className="mt-5 text-center text-sm text-gray-500 dark:text-gray-400">
        Don&apos;t have an account?{" "}
        <Link href={ROUTES.REGISTER} className="text-emerald-600 hover:underline font-medium">
          Sign up
        </Link>
      </p>
    </div>
  );
}
