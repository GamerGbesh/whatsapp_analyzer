import { useMemo } from "react";
import { addDays, format, parseISO, startOfWeek } from "date-fns";
import type { UserStat } from "@/lib/types";

// GitHub-style activity heatmap.
export function Heatmap({ stat }: { stat: UserStat }) {
  const { weeks, max, monthLabels } = useMemo(() => {
    const days = stat.messages_per_day;
    const dates = Object.keys(days).sort();
    if (dates.length === 0) {
      return { weeks: [] as { date: string; count: number }[][], max: 0, monthLabels: [] as { idx: number; label: string }[] };
    }
    const start = startOfWeek(parseISO(dates[0]), { weekStartsOn: 1 });
    const end = parseISO(dates[dates.length - 1]);
    const totalDays = Math.ceil((end.getTime() - start.getTime()) / 86400000) + 7;
    const cells: { date: string; count: number }[] = [];
    let max = 0;
    for (let i = 0; i < totalDays; i++) {
      const d = addDays(start, i);
      const key = format(d, "yyyy-MM-dd");
      const count = days[key] ?? 0;
      if (count > max) max = count;
      cells.push({ date: key, count });
    }
    const weeks: { date: string; count: number }[][] = [];
    for (let i = 0; i < cells.length; i += 7) weeks.push(cells.slice(i, i + 7));

    const monthLabels: { idx: number; label: string }[] = [];
    let lastMonth = -1;
    weeks.forEach((w, idx) => {
      const m = parseISO(w[0].date).getMonth();
      if (m !== lastMonth) {
        monthLabels.push({ idx, label: format(parseISO(w[0].date), "MMM") });
        lastMonth = m;
      }
    });
    return { weeks, max, monthLabels };
  }, [stat]);

  function intensity(c: number) {
    if (c === 0) return "oklch(0.27 0.013 200)";
    const t = Math.min(1, c / Math.max(1, max));
    // mix from muted -> primary
    const lightness = 0.3 + t * 0.45;
    const chroma = 0.04 + t * 0.12;
    return `oklch(${lightness} ${chroma} 165)`;
  }

  if (weeks.length === 0) {
    return (
      <div className="rounded-2xl border bg-panel p-5 text-sm text-muted-foreground">
        No activity to display.
      </div>
    );
  }

  return (
    <div className="rounded-2xl border bg-panel p-5 shadow-xl">
      <h3 className="mb-4 font-semibold">Activity heatmap</h3>
      <div className="overflow-x-auto">
        <div className="inline-block">
          <div className="relative ml-6 mb-1 h-3 text-[10px] text-muted-foreground">
            {monthLabels.map((m) => (
              <span
                key={`${m.idx}-${m.label}`}
                className="absolute"
                style={{ left: m.idx * 14 }}
              >
                {m.label}
              </span>
            ))}
          </div>
          <div className="flex gap-[3px]">
            {weeks.map((w, wi) => (
              <div key={wi} className="flex flex-col gap-[3px]">
                {w.map((c) => (
                  <div
                    key={c.date}
                    title={`${c.date}: ${c.count} messages`}
                    className="h-[11px] w-[11px] rounded-[2px]"
                    style={{ background: intensity(c.count) }}
                  />
                ))}
              </div>
            ))}
          </div>
          <div className="mt-3 flex items-center justify-end gap-2 text-[10px] text-muted-foreground">
            <span>Less</span>
            {[0, 0.25, 0.5, 0.75, 1].map((t) => (
              <div
                key={t}
                className="h-[11px] w-[11px] rounded-[2px]"
                style={{ background: intensity(Math.round(t * max)) }}
              />
            ))}
            <span>More</span>
          </div>
        </div>
      </div>
    </div>
  );
}
