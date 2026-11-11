import clsx from "clsx";

type Variant = "primary" | "secondary" | "ghost" | "danger";

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: "sm" | "md";
}

export function Button({
  className,
  variant = "secondary",
  size = "md",
  disabled,
  children,
  ...props
}: ButtonProps) {
  return (
    <button
      className={clsx(
        "inline-flex items-center justify-center gap-2 rounded-[var(--afs-radius)] border font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50",
        size === "sm" ? "px-2.5 py-1.5 text-xs" : "px-3 py-2 text-sm",
        variant === "primary" &&
          "border-accent-teal/50 bg-accent-teal text-graphite-950 hover:bg-accent-teal-dim hover:text-white",
        variant === "secondary" &&
          "border-graphite-700 bg-graphite-800 text-graphite-100 hover:border-accent-blue/50 hover:bg-graphite-700",
        variant === "ghost" &&
          "border-transparent bg-transparent text-graphite-300 hover:bg-graphite-800 hover:text-graphite-100",
        variant === "danger" &&
          "border-danger/40 bg-danger/10 text-danger hover:bg-danger/20",
        className,
      )}
      disabled={disabled}
      {...props}
    >
      {children}
    </button>
  );
}
