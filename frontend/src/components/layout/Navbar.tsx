"use client";

import Link from "next/link";
import { Menu } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { WalletConnect } from "@/components/wallet/WalletConnect";
import { useAuthStore } from "@/store/authStore";
import { useLogout } from "@/hooks/useAuth";
import { ROUTES } from "@/lib/constants";

interface NavbarProps {
  onMenuClick?: () => void;
}

export function Navbar({ onMenuClick }: NavbarProps) {
  const { user, isAuthenticated } = useAuthStore();
  const logout = useLogout();

  return (
    <header className="sticky top-0 z-30 flex h-16 items-center gap-4 border-b border-gray-200 bg-white px-4 md:px-6">
      <button
        onClick={onMenuClick}
        className="md:hidden text-gray-500 hover:text-gray-700"
        aria-label="Open sidebar"
      >
        <Menu className="h-6 w-6" />
      </button>

      <Link href={ROUTES.DASHBOARD} className="flex items-center gap-2 mr-auto">
        <div className="h-8 w-8 rounded-lg bg-brand-600 flex items-center justify-center">
          <span className="text-white font-bold text-sm">A</span>
        </div>
        <span className="font-semibold text-gray-900 hidden sm:block">Ajo Platform</span>
      </Link>

      <div className="flex items-center gap-3">
        <WalletConnect />

        {isAuthenticated && user ? (
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-600 hidden md:block">
              {user.display_name}
            </span>
            <Button variant="ghost" size="sm" onClick={logout}>
              Sign out
            </Button>
          </div>
        ) : (
          <div className="flex items-center gap-2">
            <Link href={ROUTES.LOGIN}>
              <Button variant="ghost" size="sm">
                Sign in
              </Button>
            </Link>
            <Link href={ROUTES.REGISTER}>
              <Button size="sm">Sign up</Button>
            </Link>
          </div>
        )}
      </div>
    </header>
  );
}
