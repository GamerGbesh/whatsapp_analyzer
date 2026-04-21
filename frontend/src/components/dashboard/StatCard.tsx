import type { LucideIcon } from "lucide-react";

type Props = {
  label: string;
  value: string;
  hint?: string;
  icon: LucideIcon;
  accent?: boolean;
};

export function StatCard({
  label,
  value,
  hint,
  icon: Icon,
  accent = false,
}: Props) {
  return (
    <div
      className={`rounded-2xl border p-4 shadow-xl transition ${
        accent
          ? "border-primary/40 bg-gradient-to-br from-primary/20 via-panel to-panel"
          : "bg-panel"
      }`}
    >
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="text-xs uppercase tracking-wider text-muted-foreground">
            {label}
          </p>
          <p className="mt-1 truncate text-lg font-semibold">{value}</p>
          {hint && <p className="mt-1 text-xs text-muted-foreground">{hint}</p>}
        </div>
        <div className="rounded-lg border border-primary/30 bg-primary/10 p-2 text-primary">
          <Icon className="h-4 w-4" />
        </div>
      </div>
    </div>
  );
}
