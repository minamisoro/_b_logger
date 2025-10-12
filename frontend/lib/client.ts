/**
 * Type-safe API client wrapper for the Blogger API
 * Uses openapi-fetch with types generated from OpenAPI spec in api.ts
 */

import openapi from 'openapi-fetch'
import type { components, paths } from './api'

// Re-export commonly used types for convenience
export type { components, paths }

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
   * @param params.limit Number of posts per page (default: 20, max: 100)
   * @param params.cursor Cursor for pagination (use last post's published_at timestamp)
   * @param params.user_id Filter by specific user ID
   * @param params.group_id Filter by user group ID
   * @returns Promise resolving to timeline response with posts array
   */
  async getTimeline(params?: {
    limit?: number
    cursor?: string | null
    user_id?: string | null
    group_id?: string | null
  }) {
    const response = await this.client.GET('/api/timeline', {
      params: { query: params || {} }
    })
    if (response.error) {
      console.error('API Error [GET /api/timeline]:', response.error)
    }
    return response
  }
}

/**
 * Users API namespace
 * Handles user information endpoints
 */
class UsersApi {
  constructor(private client: ReturnType<typeof openapi<paths>>) {}

  /**
   * Get a random user (for testing/demo purposes)
   * @returns Promise resolving to a random user
   */
  async getRandom() {
    const response = await this.client.GET('/api/users/random')
    if (response.error) {
      console.error('API Error [GET /api/users/random]:', response.error)
    }
    return response
  }
}

/**
 * User Groups API namespace
 * Handles user group management endpoints
 */
class UserGroupsApi {
  constructor(private client: ReturnType<typeof openapi<paths>>) {}

  /**
   * List user's groups
   * @returns Promise resolving to list of user groups
   */
  async list() {
    const response = await this.client.GET('/api/user-groups')
    if (response.error) {
      console.error('API Error [GET /api/user-groups]:', response.error)
    }
    return response
  }

  /**
   * Create a new user group
   * @param data Request body with group name
   * @returns Promise resolving to created user group
   */
  async create(data: components['schemas']['CreateUserGroupRequest']) {
    const response = await this.client.POST('/api/user-groups', {
      body: data
    })
    if (response.error) {
      console.error('API Error [POST /api/user-groups]:', response.error)
    }
    return response
  }

  /**
   * Update a user group
   * @param id User group ID
   * @param data Request body with updated group name
   * @returns Promise resolving to updated user group
   */
  async update(id: string, data: components['schemas']['UpdateUserGroupRequest']) {
    const response = await this.client.PUT('/api/user-groups/{id}', {
      params: { path: { id } },
      body: data
    })
    if (response.error) {
      console.error('API Error [PUT /api/user-groups/{id}]:', response.error)
    }
    return response
  }

  /**
   * Delete a user group
   * @param id User group ID
   * @returns Promise resolving when group is deleted
   */
  async delete(id: string) {
    const response = await this.client.DELETE('/api/user-groups/{id}', {
      params: { path: { id } }
    })
    if (response.error) {
      console.error('API Error [DELETE /api/user-groups/{id}]:', response.error)
    }
    return response
  }

  /**
   * Add member to user group
   * @param groupId User group ID
   * @param data Request body with user_id
   * @returns Promise resolving when member is added
   */
  async addMember(groupId: string, data: components['schemas']['AddMemberRequest']) {
    const response = await this.client.POST('/api/user-groups/{id}/members', {
      params: { path: { id: groupId } },
      body: data
    })
    if (response.error) {
      console.error('API Error [POST /api/user-groups/{id}/members]:', response.error)
    }
    return response
  }

  /**
   * Remove member from user group
   * @param groupId User group ID
   * @param userId User ID to remove
   * @returns Promise resolving when member is removed
   */
  async removeMember(groupId: string, userId: string) {
    const response = await this.client.DELETE('/api/user-groups/{id}/members/{user_id}', {
      params: { path: { id: groupId, user_id: userId } }
    })
    if (response.error) {
      console.error('API Error [DELETE /api/user-groups/{id}/members/{user_id}]:', response.error)
    }
    return response
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

  /** User Groups endpoints for managing user groups */
  public readonly userGroups: UserGroupsApi

  /** Users endpoints for user information */
  public readonly users: UsersApi

  constructor(config: ClientConfig) {
    this.client = openapi<paths>({
      baseUrl: config.baseUrl,
      headers: config.headers
    })

    // Initialize API namespaces
    this.timeline = new TimelineApi(this.client)
    this.userGroups = new UserGroupsApi(this.client)
    this.users = new UsersApi(this.client)
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
 * import { createClient } from 'blogger-lib/client'
 *
 * const client = createClient({
 *   baseUrl: 'http://localhost:8080'
 * })
 *
 * // Use hierarchical API access
 * const { data, error } = await client.timeline.getTimeline({
 *   limit: 10,
 *   cursor: null
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
