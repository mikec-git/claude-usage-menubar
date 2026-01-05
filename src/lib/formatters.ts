export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
}

export function formatTokens(count: number): string {
  if (count >= 1_000_000) {
    return `${(count / 1_000_000).toFixed(1)}M`;
  }
  if (count >= 1_000) {
    return `${(count / 1_000).toFixed(1)}K`;
  }
  return count.toString();
}

export function formatDuration(minutes: number): string {
  const hours = Math.floor(minutes / 60);
  const mins = Math.floor(minutes % 60);
  if (hours > 0) {
    return `${hours}h ${mins}m`;
  }
  return `${mins}m`;
}

export function formatTime(isoString: string): string {
  return new Date(isoString).toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
  });
}

export function formatDate(isoString: string): string {
  return new Date(isoString).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}

export function getModelDisplayName(model: string): string {
  if (model.includes("opus-4-5") || model.includes("opus-4.5")) {
    return "Opus 4.5";
  }
  if (model.includes("sonnet-4-5") || model.includes("sonnet-4.5")) {
    return "Sonnet 4.5";
  }
  if (model.includes("sonnet-4-") || model.includes("sonnet-4.")) {
    return "Sonnet 4";
  }
  if (model.includes("haiku-4-5") || model.includes("haiku-4.5")) {
    return "Haiku 4.5";
  }
  if (model.includes("sonnet")) {
    return "Sonnet 3.5";
  }
  if (model.includes("haiku")) {
    return "Haiku 3.5";
  }
  if (model.includes("opus")) {
    return "Opus 3";
  }
  return model;
}
