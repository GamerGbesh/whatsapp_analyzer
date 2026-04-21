import { Crown, Trophy } from "lucide-react";
import type { UserStat } from "@/lib/types";

type Props = {
  stats: UserStat[];
  mostActive: string | null;
  selected: string;
  onSelect: (name: string) => void;
};

export function Leaderboard({ stats, mostActive, selected, onSelect }: Props) {
  const sorted = [...stats].sort((a, b) => b.total_chats - a.total_chats);
  const max = sorted[0]?.total_chats ?? 1;

  return (
    <div className="rounded-2xl border bg-panel p-5 shadow-xl">
      <div className="mb-4 flex items-center gap-2">
        <Trophy className="h-4 w-4 text-primary" />
        <h3 className="font-semibold">Leaderboard</h3>
      </div>
      <ul className="space-y-2">
        {sorted.map((s, i) => {
          const isWinner = s.user.name === mostActive;
          const isSelected = s.user.name === selected;
          const pct = (s.total_chats / max) * 100;
          return (
            <li key={s.user.name}>
              <button
                onClick={() => onSelect(s.user.name)}
                className={`relative w-full overflow-hidden rounded-lg border bg-background/30 p-3 text-left transition hover:border-primary/60 ${
                  isSelected ? "border-primary" : "border-transparent"
                }`}
              >
                <div
                  className="absolute inset-y-0 left-0 bg-primary/15"
                  style={{ width: `${pct}%` }}
                />
                <div className="relative flex items-center justify-between gap-2">
                  <div className="flex items-center gap-2 min-w-0">
                    <span className="text-xs font-mono text-muted-foreground w-5">
                      {i + 1}
                    </span>
                    <span className="truncate font-medium">{s.user.name}</span>
                    {isWinner && <Crown className="h-3.5 w-3.5 shrink-0 text-primary" />}
                  </div>
                  <span className="font-mono text-sm tabular-nums">
                    {s.total_chats.toLocaleString()}
                  </span>
                </div>
              </button>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
