import { ref, watch } from 'vue'
import { defineStore } from 'pinia'

export type Theme = 'light' | 'dark'

export const useThemeStore = defineStore('theme', () => {
  const STORAGE_KEY = 'blogger-theme'

  // Initialize theme from localStorage or system preference
  const getInitialTheme = (): Theme => {
    const stored = localStorage.getItem(STORAGE_KEY) as Theme | null
    if (stored === 'light' || stored === 'dark') {
      return stored
    }

    // Check system preference
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      return 'dark'
    }

    return 'light'
  }

  const currentTheme = ref<Theme>(getInitialTheme())

  // Apply theme to document
  const applyTheme = (theme: Theme) => {
    document.documentElement.setAttribute('data-theme', theme)
  }

  // Initialize theme on load
  applyTheme(currentTheme.value)

  // Watch for theme changes
  watch(currentTheme, (newTheme) => {
    applyTheme(newTheme)
    localStorage.setItem(STORAGE_KEY, newTheme)
  })

  // Toggle between light and dark
  const toggleTheme = () => {
    currentTheme.value = currentTheme.value === 'light' ? 'dark' : 'light'
  }

  // Set theme explicitly
  const setTheme = (theme: Theme) => {
    currentTheme.value = theme
  }

  return {
    currentTheme,
    toggleTheme,
    setTheme,
  }
})
