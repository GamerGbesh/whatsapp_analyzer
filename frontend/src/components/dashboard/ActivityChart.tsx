import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { format, parseISO } from "date-fns";
import type { UserStat } from "@/lib/types";

const MONTH_LABELS = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

export function DailyChart({ stat }: { stat: UserStat }) {
  const data = Object.entries(stat.messages_per_day)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([date, count]) => ({ date, count }));

  return (
    <div className="rounded-2xl border bg-panel p-5 shadow-xl">
      <h3 className="mb-4 font-semibold">Messages per day</h3>
      <div className="h-64">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data} margin={{ top: 5, right: 10, left: 0, bottom: 0 }}>
            <defs>
              <linearGradient id="g1" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="oklch(0.72 0.14 165)" stopOpacity={0.55} />
                <stop offset="100%" stopColor="oklch(0.72 0.14 165)" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="oklch(0.32 0.013 200)" />
            <XAxis
              dataKey="date"
              tickFormatter={(v) => format(parseISO(v), "MMM d")}
              stroke="oklch(0.7 0.012 200)"
              fontSize={11}
              minTickGap={30}
            />
            <YAxis stroke="oklch(0.7 0.012 200)" fontSize={11} width={32} />
            <Tooltip
              contentStyle={{
                background: "oklch(0.22 0.013 200)",
                border: "1px solid oklch(0.32 0.013 200)",
                borderRadius: 8,
                fontSize: 12,
              }}
              labelFormatter={(v) => format(parseISO(v as string), "PPP")}
            />
            <Area
              type="monotone"
              dataKey="count"
              stroke="oklch(0.72 0.14 165)"
              strokeWidth={2}
              fill="url(#g1)"
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export function MonthlyChart({ stat }: { stat: UserStat }) {
  const data = Object.entries(stat.messages_per_month)
    .map(([m, count]) => ({ m: parseInt(m, 10), count }))
    .sort((a, b) => a.m - b.m)
    .map((d) => ({ month: MONTH_LABELS[d.m - 1] ?? String(d.m), count: d.count }));

  return (
    <div className="rounded-2xl border bg-panel p-5 shadow-xl">
      <h3 className="mb-4 font-semibold">Messages per month</h3>
      <div className="h-64">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={data} margin={{ top: 5, right: 10, left: 0, bottom: 0 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="oklch(0.32 0.013 200)" />
            <XAxis dataKey="month" stroke="oklch(0.7 0.012 200)" fontSize={11} />
            <YAxis stroke="oklch(0.7 0.012 200)" fontSize={11} width={32} />
            <Tooltip
              contentStyle={{
                background: "oklch(0.22 0.013 200)",
                border: "1px solid oklch(0.32 0.013 200)",
                borderRadius: 8,
                fontSize: 12,
              }}
            />
            <Bar dataKey="count" fill="oklch(0.72 0.14 165)" radius={[6, 6, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
