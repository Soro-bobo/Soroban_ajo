import Link from "next/link";
import { ArrowRight, Shield, Users, Zap } from "lucide-react";
import { Button } from "@/components/ui/Button";
import { ROUTES } from "@/lib/constants";

export default function HomePage() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-brand-50 via-white to-stellar-blue/5">
      <nav className="flex items-center justify-between px-6 py-4 max-w-7xl mx-auto">
        <div className="flex items-center gap-2">
          <div className="h-8 w-8 rounded-lg bg-brand-600 flex items-center justify-center">
            <span className="text-white font-bold text-sm">A</span>
          </div>
          <span className="font-semibold text-gray-900">Ajo Platform</span>
        </div>
        <div className="flex items-center gap-3">
          <Link href={ROUTES.LOGIN}>
            <Button variant="ghost" size="sm">Sign in</Button>
          </Link>
          <Link href={ROUTES.REGISTER}>
            <Button size="sm">Get Started</Button>
          </Link>
        </div>
      </nav>

      <section className="text-center px-6 py-24 max-w-4xl mx-auto">
        <h1 className="text-5xl sm:text-6xl font-extrabold text-gray-900 leading-tight mb-6">
          Savings Circles,{" "}
          <span className="text-brand-600">On-Chain.</span>
        </h1>
        <p className="text-xl text-gray-500 max-w-2xl mx-auto mb-10">
          Join or create decentralized Ajo savings groups. Contribute in XLM, receive
          rotating payouts, all secured by Soroban smart contracts on Stellar.
        </p>
        <div className="flex flex-col sm:flex-row justify-center gap-4">
          <Link href={ROUTES.REGISTER}>
            <Button size="lg" rightIcon={<ArrowRight className="h-5 w-5" />}>
              Start Saving
            </Button>
          </Link>
          <Link href={ROUTES.GROUPS}>
            <Button variant="secondary" size="lg">
              Browse Groups
            </Button>
          </Link>
        </div>
      </section>

      <section className="px-6 py-16 max-w-5xl mx-auto">
        <div className="grid sm:grid-cols-3 gap-6">
          {[
            {
              icon: <Shield className="h-7 w-7 text-brand-600" />,
              title: "Smart Contract Security",
              desc: "Every group, contribution, and payout is enforced on-chain by audited Soroban contracts.",
            },
            {
              icon: <Users className="h-7 w-7 text-brand-600" />,
              title: "Rotating Payouts",
              desc: "Round-robin rotation ensures every member receives their lump sum in a fair, transparent order.",
            },
            {
              icon: <Zap className="h-7 w-7 text-brand-600" />,
              title: "Stellar Speed",
              desc: "Transactions settle in 3-5 seconds on Stellar with near-zero fees.",
            },
          ].map(({ icon, title, desc }) => (
            <div
              key={title}
              className="rounded-2xl border border-gray-200 bg-white p-6 flex flex-col gap-3"
            >
              <div className="h-12 w-12 rounded-xl bg-brand-50 flex items-center justify-center">
                {icon}
              </div>
              <h3 className="font-semibold text-gray-900">{title}</h3>
              <p className="text-sm text-gray-500">{desc}</p>
            </div>
          ))}
        </div>
      </section>
    </main>
  );
}
