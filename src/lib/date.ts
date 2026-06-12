/**
 * Formats a date value into a human-readable string.
 * Handles Date objects, ISO strings, timestamps (ms), and null/undefined.
 *
 * @example
 * formatDate(new Date())           // "Feb 28, 2026, 14:32:05"
 * formatDate("2026-02-28T14:32")   // "Feb 28, 2026, 14:32:00"
 * formatDate(1709128325000)        // "Feb 28, 2026, 14:32:05"
 * formatDate(null)                 // "Never"
 */
export function formatDate(
  value: Date | string | number | null | undefined,
  options: {
    fallback?: string;
    includeTime?: boolean;
    includeSeconds?: boolean;
    locale?: string;
  } = {},
): string {
  const {
    fallback = "Never",
    includeTime = true,
    includeSeconds = true,
    locale = "en-US",
  } = options;

  if (value === null || value === undefined) return fallback;

  const date = value instanceof Date ? value : new Date(value);

  if (isNaN(date.getTime())) return fallback;

  return date.toLocaleString(locale, {
    month: "short",
    day: "numeric",
    year: "numeric",
    ...(includeTime && {
      hour: "2-digit",
      minute: "2-digit",
      ...(includeSeconds && { second: "2-digit" }),
      hour12: false,
    }),
  });
}
/**
 * Format a date in short format (e.g., "Jan 15, 2024")
 */
export function formatDateShort(date: Date | string | number): string {
  const d = new Date(date);

  const months = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ];

  const month = months[d.getMonth()];
  const day = d.getDate();
  const year = d.getFullYear();

  return `${month} ${day}, ${year}`;
}

/**
 * Format a date in very short format (e.g., "01/15/24")
 */
export function formatDateVeryShort(date: Date | string | number): string {
  const d = new Date(date);

  const month = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  const year = String(d.getFullYear()).slice(-2);

  return `${month}/${day}/${year}`;
}

/**
 * Format a date with time (e.g., "Jan 15, 2024 at 3:45 PM")
 */
export function formatDateTimeShort(date: Date | string | number): string {
  const d = new Date(date);

  const dateStr = formatDateShort(d);
  const hours = d.getHours();
  const minutes = String(d.getMinutes()).padStart(2, "0");
  const ampm = hours >= 12 ? "PM" : "AM";
  const displayHours = hours % 12 || 12;

  return `${dateStr} at ${displayHours}:${minutes} ${ampm}`;
}

/**
 * Format a date relative to now (e.g., "2 hours ago", "Yesterday", "Last week")
 */
export function formatDateRelative(date: Date | string | number): string {
  const d = new Date(date);
  const now = new Date();
  const diffMs = now.getTime() - d.getTime();
  const diffSecs = Math.floor(diffMs / 1000);
  const diffMins = Math.floor(diffSecs / 60);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffSecs < 60) {
    return "Just now";
  } else if (diffMins < 60) {
    return `${diffMins} ${diffMins === 1 ? "minute" : "minutes"} ago`;
  } else if (diffHours < 24) {
    return `${diffHours} ${diffHours === 1 ? "hour" : "hours"} ago`;
  } else if (diffDays === 1) {
    return "Yesterday";
  } else if (diffDays < 7) {
    return `${diffDays} days ago`;
  } else if (diffDays < 30) {
    const weeks = Math.floor(diffDays / 7);
    return `${weeks} ${weeks === 1 ? "week" : "weeks"} ago`;
  } else {
    return formatDateShort(d);
  }
}

/**
 * Format a date in compact format (e.g., "15 Jan" or "15 Jan 2023" if not current year)
 */
export function formatDateCompact(date: Date | string | number): string {
  const d = new Date(date);
  const now = new Date();

  const months = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ];

  const day = d.getDate();
  const month = months[d.getMonth()];
  const year = d.getFullYear();

  if (year === now.getFullYear()) {
    return `${day} ${month}`;
  } else {
    return `${day} ${month} ${year}`;
  }
}
export function formatDuration(seconds: number): string {
  if (seconds < 60) {
    return `${seconds}s`;
  }

  const units = [
    { label: "year", seconds: 365 * 24 * 60 * 60 },
    { label: "month", seconds: 30 * 24 * 60 * 60 },
    { label: "day", seconds: 24 * 60 * 60 },
    { label: "hour", seconds: 60 * 60 },
    { label: "min", seconds: 60 },
  ];

  for (const unit of units) {
    if (seconds >= unit.seconds) {
      const value = Math.floor(seconds / unit.seconds);
      const plural = value > 1 ? "s" : "";
      return `${value} ${unit.label}${plural}`;
    }
  }

  // fallback (should never reach here)
  return `${seconds}s`;
}
export function formatBigNumber(value: number): string {
  if (value < 1000) return value.toString();

  const units = ["k", "M", "B", "T"];
  let unitIndex = -1;
  let formattedValue = value;

  while (formattedValue >= 1000 && unitIndex < units.length - 1) {
    formattedValue /= 1000;
    unitIndex++;
  }

  return `${formattedValue.toFixed(1).replace(/\.0$/, "")}${units[unitIndex]}`;
}
