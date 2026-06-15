"use client";

import Link from "next/link";
import { ArrowLeft } from "lucide-react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Button } from "@/components/ui/Button";
import { Input } from "@/components/ui/Input";
import { PageHeader } from "@/components/layout/PageHeader";
import { useCreateGroup } from "@/hooks/useGroups";
import { ROUTES } from "@/lib/constants";
import type { ContributionFrequency } from "@/types";

const schema = z.object({
  name: z.string().min(2, "At least 2 characters").max(100),
  description: z.string().max(500).optional(),
  contribution_amount: z
    .string()
    .refine((v) => parseFloat(v) > 0, "Must be greater than 0"),
  frequency: z.enum(["weekly", "biweekly", "monthly"] as const),
  max_members: z
    .number({ coerce: true })
    .int()
    .min(2, "Minimum 2")
    .max(50, "Maximum 50"),
  start_date: z
    .string()
    .refine((v) => new Date(v) >= new Date(), "Start date must be in the future"),
});

type FormData = z.infer<typeof schema>;

export default function NewGroupPage() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { frequency: "monthly", max_members: 5 },
  });

  const { mutate: createGroup, isPending } = useCreateGroup();

  const today = new Date().toISOString().split("T")[0];

  return (
    <div className="max-w-xl mx-auto">
      <div className="mb-4">
        <Link href={ROUTES.GROUPS}>
          <Button variant="ghost" size="sm" leftIcon={<ArrowLeft className="h-4 w-4" />}>
            Back
          </Button>
        </Link>
      </div>

      <PageHeader
        title="Create Savings Group"
        description="Set up your Ajo circle. You'll be the first member."
      />

      <form
        onSubmit={handleSubmit((data) =>
          createGroup({
            name: data.name,
            description: data.description,
            contribution_amount: data.contribution_amount,
            frequency: data.frequency as ContributionFrequency,
            max_members: data.max_members,
            start_date: data.start_date,
          })
        )}
        className="bg-white rounded-2xl border border-gray-200 p-6 space-y-5"
      >
        <Input
          label="Group Name"
          placeholder="Lagos Savings Circle"
          error={errors.name?.message}
          required
          {...register("name")}
        />

        <div className="flex flex-col gap-1.5">
          <label className="text-sm font-medium text-gray-700">
            Description
          </label>
          <textarea
            rows={3}
            placeholder="What is this group for?"
            className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500"
            {...register("description")}
          />
          {errors.description && (
            <p className="text-xs text-red-600">{errors.description.message}</p>
          )}
        </div>

        <div className="grid grid-cols-2 gap-4">
          <Input
            label="Contribution (XLM)"
            type="number"
            step="0.0000001"
            placeholder="100"
            error={errors.contribution_amount?.message}
            required
            {...register("contribution_amount")}
          />

          <Input
            label="Max Members"
            type="number"
            min={2}
            max={50}
            error={errors.max_members?.message}
            required
            {...register("max_members")}
          />
        </div>

        <div className="flex flex-col gap-1.5">
          <label className="text-sm font-medium text-gray-700">
            Frequency <span className="text-red-500">*</span>
          </label>
          <select
            className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500"
            {...register("frequency")}
          >
            <option value="weekly">Weekly</option>
            <option value="biweekly">Bi-weekly</option>
            <option value="monthly">Monthly</option>
          </select>
          {errors.frequency && (
            <p className="text-xs text-red-600">{errors.frequency.message}</p>
          )}
        </div>

        <Input
          label="Start Date"
          type="date"
          min={today}
          error={errors.start_date?.message}
          required
          {...register("start_date")}
        />

        <Button type="submit" className="w-full" isLoading={isPending}>
          Create Group
        </Button>
      </form>
    </div>
  );
}
