import type { WhatsResult } from "./types";

// Built-in demo dataset so the dashboard is interactive before
// the user wires up their Rust API.
function makeDays(start: string, count: number, base: number, variance: number) {
  const out: Record<string, number> = {};
  const d = new Date(start);
  for (let i = 0; i < count; i++) {
    const key = d.toISOString().slice(0, 10);
    const v = Math.max(0, Math.round(base + (Math.sin(i / 3) + Math.random()) * variance));
    out[key] = v;
    d.setDate(d.getDate() + 1);
  }
  return out;
}

function monthly(days: Record<string, number>) {
  const m: Record<string, number> = {};
  for (const [k, v] of Object.entries(days)) {
    const month = String(parseInt(k.slice(5, 7), 10));
    m[month] = (m[month] ?? 0) + v;
  }
  return m;
}

function pickBusiest(days: Record<string, number>): [string, number] {
  let best: [string, number] = ["", 0];
  for (const [k, v] of Object.entries(days)) if (v > best[1]) best = [k, v];
  return best;
}

function pickBusiestMonth(months: Record<string, number>): [number, number] {
  let best: [number, number] = [1, 0];
  for (const [k, v] of Object.entries(months))
    if (v > best[1]) best = [parseInt(k, 10), v];
  return best;
}

const users = [
  { name: "Aisha", base: 18, var: 8, len: 42.3 },
  { name: "Marcus", base: 12, var: 6, len: 28.1 },
  { name: "Priya", base: 22, var: 10, len: 51.7 },
  { name: "Liam", base: 7, var: 4, len: 19.5 },
];

export const sampleResult: WhatsResult = {
  most_active_user: "Priya",
  user_results: users.map((u) => {
    const days = makeDays("2024-09-01", 120, u.base, u.var);
    const months = monthly(days);
    const total = Object.values(days).reduce((a, b) => a + b, 0);
    const dates = Object.keys(days).sort();
    return {
      user: { name: u.name },
      messages_per_day: days,
      messages_per_month: months,
      busiest_day: pickBusiest(days),
      busiest_month: pickBusiestMonth(months),
      first_message: dates[0],
      last_message: dates[dates.length - 1],
      total_chats: total,
      avg_message_length: u.len,
    };
  }),
};
