"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  LayoutDashboard,
  Users,
  Wallet,
  PlusCircle,
} from "lucide-react";
import { clsx } from "clsx";
import { ROUTES } from "@/lib/constants";

const navItems = [
  { href: ROUTES.DASHBOARD, label: "Dashboard", icon: LayoutDashboard },
  { href: ROUTES.GROUPS, label: "Groups", icon: Users },
  { href: ROUTES.GROUP_NEW, label: "New Group", icon: PlusCircle },
  { href: ROUTES.WALLET, label: "Wallet", icon: Wallet },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="flex flex-col w-64 min-h-screen border-r border-gray-200 bg-white">
      <div className="h-16 flex items-center px-6 border-b border-gray-200">
        <Link href={ROUTES.DASHBOARD} className="flex items-center gap-2">
          <div className="h-8 w-8 rounded-lg bg-brand-600 flex items-center justify-center">
            <span className="text-white font-bold text-sm">A</span>
          </div>
          <span className="font-semibold text-gray-900">Ajo Platform</span>
        </Link>
      </div>

      <nav className="flex-1 px-3 py-4 space-y-1">
        {navItems.map(({ href, label, icon: Icon }) => {
          const isActive =
            href === ROUTES.DASHBOARD
              ? pathname === href
              : pathname.startsWith(href);

          return (
            <Link
              key={href}
              href={href}
              className={clsx(
                "flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors",
                isActive
                  ? "bg-brand-50 text-brand-700"
                  : "text-gray-600 hover:bg-gray-100 hover:text-gray-900"
              )}
            >
              <Icon
                className={clsx(
                  "h-5 w-5 shrink-0",
                  isActive ? "text-brand-600" : "text-gray-400"
                )}
              />
              {label}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
