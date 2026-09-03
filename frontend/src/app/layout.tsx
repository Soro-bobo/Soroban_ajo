import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Providers } from "./providers";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: {
    template: "%s | Ajo Platform",
    default: "Ajo Platform — Decentralized Savings Groups",
  },
  description:
    "Join or create decentralized Ajo/ROSCA savings groups powered by Stellar and Soroban.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="h-full" suppressHydrationWarning>
      <body
        className={`${inter.className} h-full bg-gray-50 text-gray-900 antialiased dark:bg-gray-950 dark:text-gray-50`}
      >
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
