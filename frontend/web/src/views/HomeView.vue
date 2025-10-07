<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()

onMounted(() => {
  // Auto-redirect if logged in
  if (userStore.isLoggedIn && userStore.currentUser) {
    router.replace(`/${userStore.currentUser.id}/timeline`)
  }
})

const goToLogin = () => {
  router.push('/login')
}
</script>

<template>
  <div class="home-view">
    <div class="welcome-container">
      <h1 class="logo">_B_logger</h1>
      <p class="tagline">Your personal blogging platform</p>

      <div class="actions">
        <button class="btn btn-primary" @click="goToLogin">Get Started</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 120px); /* Account for header */
  padding: var(--spacing-lg);
}

.welcome-container {
  text-align: center;
  max-width: 600px;
}

.logo {
  font-size: 3rem;
  font-weight: 700;
  margin-bottom: var(--spacing-md);
  color: var(--text-primary);
}

.tagline {
  font-size: var(--font-size-lg);
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xl);
}

.actions {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-md);
}

.btn {
  min-width: 200px;
  padding: var(--spacing-md) var(--spacing-lg);
  font-size: var(--font-size-lg);
}
</style>
