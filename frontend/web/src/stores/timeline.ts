import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import openapi from 'openapi-fetch'
import type { components, paths } from 'blogger-lib/api'

const client = openapi<paths>({
  baseUrl: import.meta.env.VITE_API_URL || 'http://localhost:8080'
})

type TimelinePost = components['schemas']['TimelinePost']

export const useTimelineStore = defineStore('timeline', () => {
  const posts = ref<TimelinePost[]>([])
  const nextCursor = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const hasMore = computed(() => nextCursor.value !== null)

  // Filters
  const userId = ref<string | null>(null)
  const groupId = ref<string | null>(null)

  async function fetchPosts(reset = false) {
    if (loading.value) return
    if (!reset && !hasMore.value) return

    loading.value = true
    error.value = null

    try {
      const params: Record<string, string | number | undefined> = {
        limit: 20
      }

      if (!reset && nextCursor.value) {
        params.cursor = nextCursor.value
      }

      if (userId.value) {
        params.user_id = userId.value
      } else if (groupId.value) {
        params.group_id = groupId.value
      }

      const { data, error: fetchError } = await client.GET('/api/timeline', {
        params: { query: params as any }
      })

      if (fetchError) {
        error.value = fetchError.message || 'Failed to fetch timeline'
        return
      }

      if (data) {
        if (reset) {
          posts.value = data.posts
        } else {
          posts.value = [...posts.value, ...data.posts]
        }
        nextCursor.value = data.next_cursor || null
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
    } finally {
      loading.value = false
    }
  }

  function setFilter(params: { userId?: string | null; groupId?: string | null }) {
    if (params.userId !== undefined) {
      userId.value = params.userId
      groupId.value = null
    } else if (params.groupId !== undefined) {
      groupId.value = params.groupId
      userId.value = null
    }
    reset()
  }

  function reset() {
    posts.value = []
    nextCursor.value = null
    error.value = null
    fetchPosts(true)
  }

  function loadMore() {
    if (hasMore.value && !loading.value) {
      fetchPosts(false)
    }
  }

  return {
    posts,
    loading,
    error,
    hasMore,
    userId,
    groupId,
    fetchPosts,
    setFilter,
    reset,
    loadMore
  }
})
