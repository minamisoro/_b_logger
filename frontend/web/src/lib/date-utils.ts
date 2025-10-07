/**
 * Format a date as yyyy-mm-dd
 */
export function formatDate(date: Date | string): string {
  const d = typeof date === 'string' ? new Date(date) : date
  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

/**
 * Get relative date phrase from a date
 * Examples: "today", "1 day ago", "2 weeks ago", "3 months ago", "long time ago"
 */
export function getRelativeDate(date: Date | string): string {
  const d = typeof date === 'string' ? new Date(date) : date
  const now = new Date()

  // Reset time parts to compare dates only
  const dateOnly = new Date(d.getFullYear(), d.getMonth(), d.getDate())
  const nowOnly = new Date(now.getFullYear(), now.getMonth(), now.getDate())

  const diffMs = nowOnly.getTime() - dateOnly.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffDays === 0) {
    return 'today'
  } else if (diffDays === 1) {
    return '1 day ago'
  } else if (diffDays < 7) {
    return `${diffDays} days ago`
  } else if (diffDays < 14) {
    return '1 week ago'
  } else if (diffDays < 30) {
    const weeks = Math.floor(diffDays / 7)
    return `${weeks} weeks ago`
  } else if (diffDays < 60) {
    return '1 month ago'
  } else if (diffDays < 365) {
    const months = Math.floor(diffDays / 30)
    return `${months} months ago`
  } else {
    return 'long time ago'
  }
}

/**
 * Get combined date display with both absolute and relative formats
 */
export function getDateDisplay(date: Date | string): { absolute: string; relative: string } {
  return {
    absolute: formatDate(date),
    relative: getRelativeDate(date)
  }
}
