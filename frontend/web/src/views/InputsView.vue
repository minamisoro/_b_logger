<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const userId = computed(() => route.params.userId as string)
</script>

<template>
  <div class="inputs-view">
    <div class="container">
      <h1>Inputs</h1>
      <p class="user-info">Viewing inputs for user: <strong>{{ userId }}</strong></p>
      <p class="placeholder">Inputs view coming soon...</p>
    </div>
  </div>
</template>

<style scoped>
.inputs-view {
  padding: var(--spacing-lg);
}

h1 {
  color: var(--text-primary);
  margin-bottom: var(--spacing-md);
}

.user-info {
  color: var(--text-secondary);
  margin-bottom: var(--spacing-lg);
}

.placeholder {
  color: var(--text-secondary);
  font-style: italic;
}
</style>
