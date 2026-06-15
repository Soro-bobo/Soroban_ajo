"use client";

import { Menu, Bell } from "lucide-react";
import { WalletConnect } from "@/components/wallet/WalletConnect";
import { useAuthStore } from "@/store/authStore";

interface NavbarProps {
  onMenuClick?: () => void;
}

export function Navbar({ onMenuClick }: NavbarProps) {
  const { user } = useAuthStore();

  return (
    <header className="sticky top-0 z-30 flex h-16 items-center gap-4 border-b border-gray-200 bg-white px-4 md:px-6">
      {/* Mobile menu toggle */}
      <button
        onClick={onMenuClick}
        className="md:hidden text-gray-500 hover:text-gray-700 transition-colors"
        aria-label="Open sidebar"
      >
        <Menu className="h-6 w-6" />
      </button>

      {/* Page title slot — blank here, filled by breadcrumbs on each page */}
      <div className="flex-1" />

      {/* Right-side actions */}
      <div className="flex items-center gap-3">
        <WalletConnect />

        <button
          className="hidden sm:flex items-center justify-center h-9 w-9 rounded-lg text-gray-400 hover:bg-gray-100 hover:text-gray-600 transition-colors"
          aria-label="Notifications"
        >
          <Bell className="h-5 w-5" />
        </button>

        {user && (
          <div className="flex items-center gap-2">
            <div className="h-8 w-8 rounded-full bg-emerald-100 flex items-center justify-center">
              <span className="text-emerald-700 font-semibold text-sm">
                {user.display_name.charAt(0).toUpperCase()}
              </span>
            </div>
            <span className="text-sm font-medium text-gray-700 hidden md:block">
              {user.display_name}
            </span>
          </div>
        )}
      </div>
    </header>
  );
}
