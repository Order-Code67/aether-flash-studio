import clsx from "clsx";
import { useT } from "@/i18n";
import type { MenuSubItem } from "@/navigation/menu";

interface SectionTabsProps {
  items: MenuSubItem[];
  active: string;
  onChange: (id: string) => void;
}

export function SectionTabs({ items, active, onChange }: SectionTabsProps) {
  const t = useT();
  return (
    <div className="flex flex-wrap gap-1 border-b border-graphite-700 px-4 pt-3">
      {items.map((item) => (
        <button
          key={item.id}
          className={clsx(
            "border-b-2 px-3 py-2 text-xs font-medium transition-colors",
            active === item.id
              ? "border-accent-teal text-accent-teal"
              : "border-transparent text-graphite-400 hover:text-graphite-100",
          )}
          onClick={() => onChange(item.id)}
        >
          {t(item.labelKey)}
        </button>
      ))}
    </div>
  );
}
