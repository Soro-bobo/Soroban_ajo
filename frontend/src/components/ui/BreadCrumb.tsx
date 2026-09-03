"use client";

import Link from "next/link";
import { ChevronRight, Home } from "lucide-react";
import { clsx } from "clsx";

export interface BreadCrumbItem {
  label: string;
  href?: string;
}

interface BreadCrumbProps {
  items: BreadCrumbItem[];
  className?: string;
}

export function BreadCrumb({ items, className }: BreadCrumbProps) {
  return (
    <nav aria-label="Breadcrumb" className={clsx("flex items-center gap-1 text-sm", className)}>
      <Link
        href="/dashboard"
        className="flex items-center text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 transition-colors"
        aria-label="Home"
      >
        <Home className="h-3.5 w-3.5" />
      </Link>

      {items.map((item, idx) => {
        const isLast = idx === items.length - 1;
        return (
          <span key={idx} className="flex items-center gap-1">
            <ChevronRight className="h-3.5 w-3.5 text-gray-300 dark:text-gray-700 shrink-0" />
            {isLast || !item.href ? (
              <span
                className={clsx(
                  "truncate max-w-[160px]",
                  isLast ? "text-gray-700 dark:text-gray-200 font-medium" : "text-gray-400 dark:text-gray-500"
                )}
                aria-current={isLast ? "page" : undefined}
              >
                {item.label}
              </span>
            ) : (
              <Link
                href={item.href}
                className="text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 transition-colors truncate max-w-[160px]"
              >
                {item.label}
              </Link>
            )}
          </span>
        );
      })}
    </nav>
  );
}
