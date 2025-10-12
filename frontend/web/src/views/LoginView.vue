<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()

const userId = ref('')
const userName = ref('')
const error = ref('')

const handleLogin = () => {
  error.value = ''

  // Validate inputs
  if (!userId.value.trim()) {
    error.value = 'User ID is required'
    return
  }

  if (!userName.value.trim()) {
    error.value = 'User name is required'
    return
  }

  // Mock login
  userStore.login({
    id: userId.value.trim(),
    username: userId.value.trim(),
    name: userName.value.trim(),
    avatarUrl: `https://i.pravatar.cc/150?u=${userId.value}`,
  })

  // Redirect to user's timeline
  router.push(`/${userId.value.trim()}/timeline`)
}

const useMockLogin = async () => {
  await userStore.mockLogin()
  if (userStore.currentUser) {
    router.push(`/${userStore.currentUser.username}/timeline`)
  }
}
</script>

<template>
  <div class="login-view">
    <div class="login-container">
      <h1>Login to _B_logger</h1>

      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label for="userId">User ID</label>
          <input
            id="userId"
            v-model="userId"
            type="text"
            placeholder="Enter your user ID"
            autocomplete="username"
          />
        </div>

        <div class="form-group">
          <label for="userName">Name</label>
          <input
            id="userName"
            v-model="userName"
            type="text"
            placeholder="Enter your name"
            autocomplete="name"
          />
        </div>

        <p v-if="error" class="error">{{ error }}</p>

        <button type="submit" class="btn btn-primary">Login</button>
      </form>

      <div class="divider">
        <span>or</span>
      </div>

      <button @click="useMockLogin" class="btn">Use Demo Account</button>
    </div>
  </div>
</template>

<style scoped>
.login-view {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 120px);
  padding: var(--spacing-lg);
}

.login-container {
  width: 100%;
  max-width: 400px;
  padding: var(--spacing-xl);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--bg-primary);
  box-shadow: var(--shadow-md);
}

h1 {
  text-align: center;
  margin-bottom: var(--spacing-xl);
  color: var(--text-primary);
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

label {
  font-weight: 500;
  color: var(--text-primary);
}

input {
  width: 100%;
}

.error {
  color: #d32f2f;
  font-size: var(--font-size-sm);
  margin: 0;
}

.btn {
  width: 100%;
}

.divider {
  display: flex;
  align-items: center;
  margin: var(--spacing-lg) 0;
  color: var(--text-secondary);
}

.divider::before,
.divider::after {
  content: '';
  flex: 1;
  border-bottom: 1px solid var(--border-color);
}

.divider span {
  padding: 0 var(--spacing-md);
}
</style>
