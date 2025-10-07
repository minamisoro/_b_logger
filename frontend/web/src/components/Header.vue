<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const searchQuery = ref('')
const showAdvancedSearch = ref(false)

// Get the current userId from route or logged-in user
const currentUserId = computed(() => {
  const routeUserId = route.params.userId as string | undefined
  return routeUserId || userStore.currentUser?.id || null
})

// Navigation items
const navItems = [
  { name: 'Timeline', key: 'timeline' },
  { name: 'Network', key: 'network' },
  { name: 'Inputs', key: 'inputs' },
  { name: 'Published', key: 'published' }
]

// Check if a nav item is active
const isActive = (key: string) => {
  return route.name === key
}

// Navigate to a view
const navigateTo = (key: string) => {
  if (currentUserId.value) {
    router.push(`/${currentUserId.value}/${key}`)
  }
}

// Handle search
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    console.log('Search query:', searchQuery.value)
    // TODO: Implement search functionality
  }
}

// Toggle advanced search
const toggleAdvancedSearch = () => {
  showAdvancedSearch.value = !showAdvancedSearch.value
}

// Handle login/logout
const handleAuthAction = () => {
  if (userStore.isLoggedIn) {
    userStore.logout()
    router.push('/')
  } else {
    router.push('/login')
  }
}

// Navigate to home
const goHome = () => {
  router.push('/')
}
</script>

<template>
  <header class="header">
    <!-- First Row: Logo, Search, Auth -->
    <div class="row primary">
      <div class="container">
        <!-- Logo -->
        <div class="logo" @click="goHome">
          <span class="text">_B_logger</span>
        </div>

        <!-- Search -->
        <div class="search">
          <form @submit.prevent="handleSearch" class="form">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search..."
              class="input"
            />
            <button type="submit" class="button">
              <i class="mdi mdi-magnify"></i>
            </button>
          </form>
          <button @click="toggleAdvancedSearch" class="advanced-toggle">
            <i class="mdi mdi-tune"></i>
            Advanced Search
          </button>

          <!-- Advanced Search Panel -->
          <div v-if="showAdvancedSearch" class="advanced-panel">
            <p>Advanced search options coming soon...</p>
          </div>
        </div>

        <!-- Auth / User -->
        <div class="auth">
          <button
            v-if="!userStore.isLoggedIn"
            @click="handleAuthAction"
            class="btn btn-primary"
          >
            Login
          </button>
          <div v-else class="user-menu">
            <div class="avatar" @click="handleAuthAction" title="Logout">
              <img
                v-if="userStore.currentUser?.avatarUrl"
                :src="userStore.currentUser.avatarUrl"
                :alt="userStore.currentUser.name"
              />
              <div v-else class="placeholder">
                {{ userStore.currentUser?.name?.charAt(0).toUpperCase() }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Second Row: Navigation -->
    <div class="row nav">
      <div class="container">
        <nav class="navigation">
          <button
            v-for="item in navItems"
            :key="item.key"
            @click="navigateTo(item.key)"
            :class="['item', { active: isActive(item.key) }]"
            :disabled="!currentUserId"
          >
            {{ item.name }}
          </button>
        </nav>
      </div>
    </div>
  </header>
</template>

<style scoped>
.header {
  position: sticky;
  top: 0;
  z-index: 100;
  box-shadow: var(--shadow-md);
}

/* Row Styles */
.header .row {
  border-bottom: 1px solid var(--border-color);
}

.header .row.primary {
  background-color: var(--bg-primary);
}

.header .row.nav {
  background-color: var(--accent-blue);
}

/* Container */
.header .row .container {
  padding: 0 var(--spacing-md);
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
}

.header .row.primary .container {
  height: 64px;
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: var(--spacing-lg);
  align-items: center;
  max-width: 100%;
  width: 100%;
}

.header .row.nav .container {
  height: 48px;
  max-width: 1200px;
  margin: 0 auto;
}

/* Logo */
.header .logo {
  cursor: pointer;
  flex-shrink: 0;
}

.header .logo .text {
  font-size: var(--font-size-xl);
  font-weight: 700;
  color: var(--text-primary);
}

.header .logo .text:hover {
  color: var(--accent-blue);
}

/* Search */
.header .search {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  max-width: 600px;
  width: 100%;
  justify-self: center;
}

.header .search .form {
  display: flex;
  gap: var(--spacing-xs);
  width: 100%;
}

.header .search .form .input {
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-size: var(--font-size-base);
}

.header .search .form .input:focus {
  outline: none;
  border-color: var(--accent-blue);
}

.header .search .form .button {
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--border-color);
  background-color: var(--button-bg);
  color: var(--text-primary);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: background-color var(--transition-fast);
}

.header .search .form .button:hover {
  background-color: var(--button-hover-bg);
}

.header .search .advanced-toggle {
  margin-top: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border: none;
  background: transparent;
  color: var(--accent-blue);
  cursor: pointer;
  font-size: var(--font-size-sm);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.header .search .advanced-toggle:hover {
  text-decoration: underline;
}

.header .search .advanced-panel {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: var(--spacing-sm);
  padding: var(--spacing-md);
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  box-shadow: var(--shadow-md);
}

/* Auth */
.header .auth {
  justify-self: end;
}

.header .auth .user-menu {
  display: flex;
  align-items: center;
}

.header .auth .user-menu .avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  cursor: pointer;
  border: 2px solid var(--border-color);
  transition: border-color var(--transition-fast);
}

.header .auth .user-menu .avatar:hover {
  border-color: var(--accent-blue);
}

.header .auth .user-menu .avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.header .auth .user-menu .avatar .placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--accent-blue);
  color: var(--accent-blue-text);
  font-weight: 600;
  font-size: 1.2rem;
}

/* Navigation */
.header .navigation {
  display: flex;
  gap: var(--spacing-xs);
  justify-content: center;
  width: 100%;
}

.header .navigation .item {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.9);
  font-size: var(--font-size-base);
  font-weight: 500;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color var(--transition-fast);
}

.header .navigation .item:hover:not(:disabled) {
  background-color: var(--accent-blue-hover);
  color: var(--accent-blue-text);
}

.header .navigation .item.active {
  background-color: rgba(255, 255, 255, 0.2);
  color: var(--accent-blue-text);
}

.header .navigation .item:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
