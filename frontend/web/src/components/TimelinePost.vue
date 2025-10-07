<script setup lang="ts">
import { ref, computed } from 'vue'
import type { components } from 'blogger-lib/api'
import { getCategoryConfig } from '@/lib/timeline'
import { getDateDisplay } from '@/lib/date-utils'

type TimelinePost = components['schemas']['TimelinePost']

const props = defineProps<{
  post: TimelinePost
}>()

const expanded = ref(false)

const categoryConfig = computed(() => getCategoryConfig(props.post.category))
const dateDisplay = computed(() => getDateDisplay(props.post.published_at))

function toggleExpand() {
  expanded.value = !expanded.value
}
</script>

<template>
  <div class="timeline-post">
    <!-- Timeline dot with icon -->
    <div class="timeline-dot" :style="{ backgroundColor: categoryConfig.color }">
      <span class="category-icon">{{ categoryConfig.icon }}</span>
    </div>

    <!-- Post content -->
    <div class="post-content">
      <!-- Date display -->
      <div class="post-date">
        <div class="date-absolute">{{ dateDisplay.absolute }}</div>
        <div class="date-relative">{{ dateDisplay.relative }}</div>
      </div>

      <!-- Post card -->
      <div class="post-card" @click="toggleExpand">
        <!-- Title (colored background) -->
        <div class="post-title" :style="{ backgroundColor: categoryConfig.color }">
          <h3>{{ post.title }}</h3>
          <span class="expand-icon">{{ expanded ? '▼' : '▶' }}</span>
        </div>

        <!-- Content (white background, expandable) -->
        <div v-if="expanded" class="post-body">
          <div class="post-text">{{ post.content }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.timeline-post {
  display: flex;
  gap: var(--spacing-md);
  position: relative;
}

.timeline-dot {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 2;
  border: 3px solid var(--color-background);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.category-icon {
  font-size: 20px;
}

.post-content {
  flex: 1;
  min-width: 0;
}

.post-date {
  margin-bottom: var(--spacing-sm);
  font-size: 0.875rem;
}

.date-absolute {
  color: var(--text-primary);
  font-weight: 500;
}

.date-relative {
  color: var(--text-secondary);
  font-size: 0.75rem;
  margin-top: 2px;
}

.post-card {
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: box-shadow 0.2s;
}

.post-card:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.post-title {
  padding: var(--spacing-md);
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.post-title h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  flex: 1;
}

.expand-icon {
  margin-left: var(--spacing-sm);
  opacity: 0.8;
  font-size: 0.875rem;
}

.post-body {
  background: white;
  padding: var(--spacing-md);
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.post-text {
  color: var(--text-primary);
  line-height: 1.6;
  white-space: pre-wrap;
}
</style>
