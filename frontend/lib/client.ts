/**
 * Type-safe hierarchical API client wrapper for the Blogger API
 * Uses openapi-fetch with types generated from OpenAPI spec in api.ts
 */

import openapi from 'openapi-fetch'
import type { components, paths } from './api'

// Re-export commonly used types for convenience
export type ApiError = components['schemas']['ApiError']
export type TimelinePost = components['schemas']['TimelinePost']
export type GetTimelineResponse = components['schemas']['GetTimelineResponse']

/**
 * Configuration for the API client
 */
export interface ClientConfig {
  /** Base URL for the API (e.g., 'http://localhost:8080') */
  baseUrl: string
  /** Optional headers to include with every request */
  headers?: HeadersInit
}

/**
 * Timeline API namespace
 * Handles endpoints related to timeline and post listings
 */
class TimelineApi {
  constructor(private client: ReturnType<typeof openapi<paths>>) {}

  /**
   * Get timeline of published posts
   * @param params Query parameters
   * @param params.limit Number of posts per page
   * @param params.pages Page number (0-indexed)
   * @returns Promise resolving to timeline response with posts array
   *
   * @example
   * ```typescript
   * const response = await client.timeline.getTimeline({ limit: 10, pages: 0 })
   * if (response.data) {
   *   console.log(response.data.posts)
   * }
   * ```
   */
  async getTimeline(params: { limit: number; pages: number }) {
    return this.client.GET('/api/timeline', {
      params: {
        query: params,
      },
    })
  }
}

/**
 * Main Blogger API Client
 * Provides type-safe access to all API endpoints organized hierarchically
 */
export class BloggerApiClient {
  private client: ReturnType<typeof openapi<paths>>

  /** Timeline endpoints for retrieving published posts */
  public readonly timeline: TimelineApi

  constructor(config: ClientConfig) {
    this.client = openapi<paths>({
      baseUrl: config.baseUrl,
      headers: config.headers,
    })

    // Initialize API namespaces
    this.timeline = new TimelineApi(this.client)
  }
}

/**
 * Create a new Blogger API client instance
 *
 * @param config Client configuration with baseUrl and optional headers
 * @returns Configured API client with hierarchical namespace access
 *
 * @example
 * ```typescript
 * import { createClient } from './lib/client'
 *
 * const client = createClient({
 *   baseUrl: 'http://localhost:8080'
 * })
 *
 * // Use hierarchical API access
 * const { data, error } = await client.timeline.getTimeline({
 *   limit: 10,
 *   pages: 0
 * })
 *
 * if (error) {
 *   console.error('API Error:', error)
 * } else {
 *   console.log('Posts:', data.posts)
 * }
 * ```
 */
export function createClient(config: ClientConfig): BloggerApiClient {
  return new BloggerApiClient(config)
}
