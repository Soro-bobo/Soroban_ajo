import Link from "next/link";
import { ArrowRight, Shield, Users, Zap } from "lucide-react";
import { Button } from "@/components/ui/Button";
import GhostFibers from "@/components/effects/GhostFibers";
import { ROUTES } from "@/lib/constants";

export default function HomePage() {
  return (
    <main className="relative h-[100dvh] overflow-hidden bg-[#120f17]">
      <GhostFibers
        lineColor="#34d399"
        glowColor="#065f46"
        speed={0.2}
        scale={2}
        rotationSpeed={0.15}
        layers={5}
        waveAmplitude={0.015}
        waveFrequency={3}
        waveSpeed={0.15}
        layerSpeed={0.08}
        twist={0.1}
        twistFrequency={5}
        twistSpeed={1.2}
        lineFrequency={5}
        lineSpacing={2}
        lineSharpness={16}
        glowFalloff={10}
        glowIntensity={1.6}
        brightness={1.8}
        blueBoost={1}
        vignette={0.85}
        grain={0.05}
      />

      <div className="relative z-10 flex h-full flex-col">
        <nav className="shrink-0 border-b border-white/5 bg-[#120f17]/70 backdrop-blur-md">
          <div className="flex items-center justify-between px-6 py-3 max-w-7xl mx-auto">
            <div className="flex items-center gap-2">
              <div className="h-7 w-7 rounded-lg bg-emerald-500 flex items-center justify-center">
                <span className="text-white font-bold text-sm">A</span>
              </div>
              <span className="font-bold text-white">Ajo Platform</span>
            </div>
            <div className="flex items-center gap-3">
              <Link href={ROUTES.LOGIN}>
                <Button
                  variant="ghost"
                  size="sm"
                  className="font-semibold text-gray-100 hover:bg-white/10 hover:text-white"
                >
                  Sign in
                </Button>
              </Link>
              <Link href={ROUTES.REGISTER}>
                <Button size="sm" className="font-semibold">
                  Get Started
                </Button>
              </Link>
            </div>
          </div>
        </nav>

        <div className="flex flex-1 min-h-0 flex-col items-center justify-center gap-[3vh] px-6 py-[2vh]">
          <section className="relative text-center max-w-4xl mx-auto shrink-0">
            <div
              className="pointer-events-none absolute inset-0 -z-10"
              style={{
                background:
                  "radial-gradient(ellipse 80% 60% at 50% 40%, rgba(0,0,0,0.55) 0%, rgba(0,0,0,0) 70%)",
              }}
            />
            <h1 className="text-[clamp(1.75rem,3vh+1rem,3.5rem)] font-extrabold text-white leading-tight mb-[1.5vh] drop-shadow-[0_2px_16px_rgba(0,0,0,0.6)]">
              Savings Circles,{" "}
              <span className="text-amber-300">On-Chain.</span>
            </h1>
            <p className="text-[clamp(0.85rem,1vh+0.6rem,1.25rem)] font-medium text-gray-100 max-w-2xl mx-auto mb-[2vh] drop-shadow-[0_1px_8px_rgba(0,0,0,0.5)]">
              Join or create decentralized Ajo savings groups. Contribute in XLM, receive
              rotating payouts, all secured by Soroban smart contracts on Stellar.
            </p>
            <div className="flex flex-row justify-center gap-3">
              <Link href={ROUTES.REGISTER}>
                <Button
                  className="font-semibold"
                  rightIcon={<ArrowRight className="h-4 w-4" />}
                >
                  Start Saving
                </Button>
              </Link>
              <Link href={ROUTES.GROUPS}>
                <Button variant="secondary" className="font-semibold">
                  Browse Groups
                </Button>
              </Link>
            </div>
          </section>

          <section className="w-full max-w-5xl mx-auto shrink-0">
            <div className="grid grid-cols-3 gap-3 sm:gap-6">
              {[
                {
                  icon: <Shield className="h-[min(1.75rem,3vh)] w-[min(1.75rem,3vh)] text-amber-300" />,
                  title: "Smart Contract Security",
                  desc: "Every group, contribution, and payout is enforced on-chain by audited Soroban contracts.",
                },
                {
                  icon: <Users className="h-[min(1.75rem,3vh)] w-[min(1.75rem,3vh)] text-amber-300" />,
                  title: "Rotating Payouts",
                  desc: "Round-robin rotation ensures every member receives their lump sum in a fair, transparent order.",
                },
                {
                  icon: <Zap className="h-[min(1.75rem,3vh)] w-[min(1.75rem,3vh)] text-amber-300" />,
                  title: "Stellar Speed",
                  desc: "Transactions settle in 3-5 seconds on Stellar with near-zero fees.",
                },
              ].map(({ icon, title, desc }) => (
                <div
                  key={title}
                  className="rounded-xl sm:rounded-2xl border border-white/10 bg-black/40 backdrop-blur-md p-2.5 sm:p-6 flex flex-col gap-1.5 sm:gap-3 shadow-[0_4px_24px_rgba(0,0,0,0.35)]"
                >
                  <div className="h-8 w-8 sm:h-12 sm:w-12 rounded-lg sm:rounded-xl bg-amber-400/10 flex items-center justify-center">
                    {icon}
                  </div>
                  <h3 className="text-[clamp(0.7rem,0.8vh+0.5rem,1rem)] font-bold text-white leading-tight">
                    {title}
                  </h3>
                  <p className="text-[clamp(0.6rem,0.7vh+0.35rem,0.875rem)] font-medium text-gray-200 line-clamp-2 sm:line-clamp-none">
                    {desc}
                  </p>
                </div>
              ))}
            </div>
          </section>
        </div>
      </div>
    </main>
  );
}
