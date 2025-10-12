import { ref, computed, watch } from 'vue'
import { defineStore } from 'pinia'
import { createClient } from 'blogger-lib/client'

const client = createClient({
  baseUrl: import.meta.env.VITE_API_URL || 'http://localhost:8080'
})

export interface User {
  id: string
  username: string
  name: string
  avatarUrl?: string
}

export const useUserStore = defineStore('user', () => {
  const STORAGE_KEY = 'blogger-user'

  // Initialize from localStorage if available
  const getInitialUser = (): User | null => {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored) {
      try {
        const parsed = JSON.parse(stored)
        // Validate the structure
        if (parsed && typeof parsed.id === 'string' && typeof parsed.username === 'string' && typeof parsed.name === 'string') {
          return parsed as User
        }
        // Invalid structure, clear it
        localStorage.removeItem(STORAGE_KEY)
      } catch {
        // Invalid JSON, clear it
        localStorage.removeItem(STORAGE_KEY)
      }
    }
    return null
  }

  const currentUser = ref<User | null>(getInitialUser())
  const isLoggedIn = computed(() => currentUser.value !== null)

  // Watch for user changes and persist to localStorage
  watch(currentUser, (newUser) => {
    if (newUser) {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(newUser))
    } else {
      localStorage.removeItem(STORAGE_KEY)
    }
  })

  // Login (mocked for now)
  const login = (user: User) => {
    currentUser.value = user
  }

  // Logout
  const logout = () => {
    currentUser.value = null
  }

  // Get effective user ID (from route param or logged-in user)
  const getEffectiveUserId = (routeUserId?: string): string | null => {
    if (routeUserId) {
      return routeUserId
    }
    return currentUser.value?.id || null
  }

  // Mock login for development (can be called from UI)
  const mockLogin = async () => {
    // Fetch a random user from the database
    const { data, error } = await client.users.getRandom()

    if (error) {
      console.error('Failed to fetch random user:', error)
      return
    }

    if (data) {
      // Generate a random avatar for the user
      const avatarId = Math.floor(Math.random() * 70) + 1

      login({
        id: data.id,
        username: data.username,
        name: data.display_name || data.username,
        avatarUrl: `https://i.pravatar.cc/150?img=${avatarId}`,
      })
    }
  }

  return {
    currentUser,
    isLoggedIn,
    login,
    logout,
    getEffectiveUserId,
    mockLogin,
  }
})
