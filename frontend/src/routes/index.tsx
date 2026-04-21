import { createFileRoute } from "@tanstack/react-router";
import { useMemo, useState } from "react";
import { format, parseISO } from "date-fns";
import {
  Crown,
  MessageSquare,
  Calendar,
  CalendarDays,
  Type,
  Clock,
  Activity,
} from "lucide-react";
import { UploadCard } from "@/components/dashboard/UploadCard";
import { StatCard } from "@/components/dashboard/StatCard";
import { Leaderboard } from "@/components/dashboard/Leaderboard";
import { DailyChart, MonthlyChart } from "@/components/dashboard/ActivityChart";
import { Heatmap } from "@/components/dashboard/Heatmap";
import { sampleResult } from "@/lib/sample";
import type { WhatsResult } from "@/lib/types";

export const Route = createFileRoute("/")({
  component: Dashboard,
  head: () => ({
    meta: [
      { title: "WhatsInsight — WhatsApp chat dashboard" },
      {
        name: "description",
        content:
          "Upload a WhatsApp chat export and explore activity, leaderboards, and heatmaps powered by your Rust analyzer.",
      },
    ],
  }),
});

const MONTH_LABELS = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December",
];

function Dashboard() {
  const [result, setResult] = useState<WhatsResult | null>(null);
  const [selected, setSelected] = useState<string>("");

  const selectedStat = useMemo(() => {
    if (!result) return null;
    return (
      result.user_results.find((s) => s.user.name === selected) ??
      result.user_results.find((s) => s.user.name === result.most_active_user) ??
      result.user_results[0] ??
      null
    );
  }, [result, selected]);

  function handleResult(r: WhatsResult) {
    setResult(r);
    setSelected(r.most_active_user ?? r.user_results[0]?.user.name ?? "");
  }

  return (
    <main className="mx-auto max-w-7xl px-4 py-8 md:py-12">
      <header className="mb-8 flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-primary-foreground shadow-lg shadow-primary/30">
            <MessageSquare className="h-5 w-5" />
          </div>
          <div>
            <h1 className="text-2xl font-bold tracking-tight">WhatsInsight</h1>
            <p className="text-xs text-muted-foreground">
              WhatsApp chat analytics · powered by Rust
            </p>
          </div>
        </div>
        {result && (
          <button
            onClick={() => setResult(null)}
            className="text-xs text-muted-foreground hover:text-foreground"
          >
            ← New upload
          </button>
        )}
      </header>

      {!result && (
        <div className="grid gap-6 md:grid-cols-2">
          <UploadCard onResult={handleResult} onLoadDemo={() => handleResult(sampleResult)} />
          <div className="rounded-2xl border bg-panel/60 p-6 shadow-xl">
            <h2 className="mb-3 text-lg font-semibold">What you'll see</h2>
            <ul className="space-y-3 text-sm text-muted-foreground">
              <li className="flex gap-3">
                <Crown className="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                <span><b className="text-foreground">Most active user</b> — crowned at the top of the leaderboard.</span>
              </li>
              <li className="flex gap-3">
                <Activity className="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                <span><b className="text-foreground">Activity over time</b> — daily area chart and monthly bars.</span>
              </li>
              <li className="flex gap-3">
                <CalendarDays className="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                <span><b className="text-foreground">Heatmap</b> — GitHub-style calendar of busiest days.</span>
              </li>
              <li className="flex gap-3">
                <Type className="mt-0.5 h-4 w-4 shrink-0 text-primary" />
                <span><b className="text-foreground">Per-user deep dive</b> — first/last message, busiest day & month, average length.</span>
              </li>
            </ul>
          </div>
        </div>
      )}

      {result && selectedStat && (
        <div className="space-y-6">
          {/* Top stat row */}
          <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
            <StatCard
              label="Most active"
              value={result.most_active_user ?? "—"}
              hint="Across all participants"
              icon={Crown}
              accent
            />
            <StatCard
              label="Total messages"
              value={result.user_results
                .reduce((a, s) => a + s.total_chats, 0)
                .toLocaleString()}
              hint={`${result.user_results.length} users`}
              icon={MessageSquare}
            />
            <StatCard
              label="First message"
              value={
                result.user_results
                  .map((s) => s.first_message)
                  .filter((d): d is string => !!d)
                  .sort()[0]
                  ? format(
                      parseISO(
                        result.user_results
                          .map((s) => s.first_message)
                          .filter((d): d is string => !!d)
                          .sort()[0],
                      ),
                      "MMM d, yyyy",
                    )
                  : "—"
              }
              icon={Calendar}
            />
            <StatCard
              label="Last message"
              value={
                result.user_results
                  .map((s) => s.last_message)
                  .filter((d): d is string => !!d)
                  .sort()
                  .reverse()[0]
                  ? format(
                      parseISO(
                        result.user_results
                          .map((s) => s.last_message)
                          .filter((d): d is string => !!d)
                          .sort()
                          .reverse()[0],
                      ),
                      "MMM d, yyyy",
                    )
                  : "—"
              }
              icon={Clock}
            />
          </div>

          {/* Two-column layout */}
          <div className="grid gap-6 lg:grid-cols-[300px_1fr]">
            <Leaderboard
              stats={result.user_results}
              mostActive={result.most_active_user}
              selected={selectedStat.user.name}
              onSelect={setSelected}
            />

            <div className="space-y-6">
              {/* Selected user header */}
              <div className="rounded-2xl border bg-gradient-to-br from-primary/15 via-panel to-panel p-5 shadow-xl">
                <div className="flex items-center justify-between gap-4">
                  <div>
                    <p className="text-xs uppercase tracking-wider text-muted-foreground">
                      Viewing
                    </p>
                    <h2 className="mt-1 text-2xl font-bold">
                      {selectedStat.user.name}
                      {selectedStat.user.name === result.most_active_user && (
                        <Crown className="ml-2 inline h-5 w-5 text-primary" />
                      )}
                    </h2>
                  </div>
                  <div className="text-right">
                    <p className="text-xs text-muted-foreground">Total chats</p>
                    <p className="text-2xl font-mono font-semibold tabular-nums">
                      {selectedStat.total_chats.toLocaleString()}
                    </p>
                  </div>
                </div>
                <div className="mt-4 grid grid-cols-2 gap-3 sm:grid-cols-4">
                  <MiniStat
                    label="Busiest day"
                    value={
                      selectedStat.busiest_day
                        ? format(parseISO(selectedStat.busiest_day[0]), "MMM d")
                        : "—"
                    }
                    sub={selectedStat.busiest_day ? `${selectedStat.busiest_day[1]} msgs` : undefined}
                  />
                  <MiniStat
                    label="Busiest month"
                    value={
                      selectedStat.busiest_month
                        ? MONTH_LABELS[selectedStat.busiest_month[0] - 1] ??
                          String(selectedStat.busiest_month[0])
                        : "—"
                    }
                    sub={selectedStat.busiest_month ? `${selectedStat.busiest_month[1]} msgs` : undefined}
                  />
                  <MiniStat
                    label="Avg length"
                    value={`${selectedStat.avg_message_length.toFixed(1)}`}
                    sub="characters"
                  />
                  <MiniStat
                    label="Active span"
                    value={
                      selectedStat.first_message && selectedStat.last_message
                        ? `${Math.max(
                            1,
                            Math.round(
                              (parseISO(selectedStat.last_message).getTime() -
                                parseISO(selectedStat.first_message).getTime()) /
                                86400000,
                            ),
                          )} d`
                        : "—"
                    }
                  />
                </div>
              </div>

              <Heatmap stat={selectedStat} />

              <div className="grid gap-6 xl:grid-cols-2">
                <DailyChart stat={selectedStat} />
                <MonthlyChart stat={selectedStat} />
              </div>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

function MiniStat({ label, value, sub }: { label: string; value: string; sub?: string }) {
  return (
    <div className="rounded-lg border bg-background/40 p-3">
      <p className="text-[10px] uppercase tracking-wider text-muted-foreground">{label}</p>
      <p className="mt-1 truncate font-semibold">{value}</p>
      {sub && <p className="text-xs text-muted-foreground">{sub}</p>}
    </div>
  );
}
