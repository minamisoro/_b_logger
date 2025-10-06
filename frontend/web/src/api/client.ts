/**
 * Global API client instance for the Blogger web frontend
 * Configured using environment variables with sensible defaults
 */

import { createClient } from '@lib/client'

/**
 * Get the API base URL from environment variables
 * Falls back to localhost:8080 for development
 */
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080'

/**
 * Global singleton instance of the Blogger API client
 * Use this throughout the application instead of creating new instances
 *
 * @example
 * ```typescript
 * import { apiClient } from '@/api/client'
 *
 * const { data, error } = await apiClient.timeline.getTimeline({
 *   limit: 10,
 *   pages: 0
 * })
 * ```
 */
export const apiClient = createClient({
  baseUrl: API_BASE_URL,
})
