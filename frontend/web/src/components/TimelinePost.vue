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
    <!-- Date display (left of icon) -->
    <div class="post-date">
      <div class="date-absolute">{{ dateDisplay.absolute }}</div>
      <div class="date-relative">{{ dateDisplay.relative }}</div>
    </div>

    <!-- Timeline dot with avatar -->
    <div class="timeline-dot">
      <img
        v-if="post.author_avatar_url"
        :src="post.author_avatar_url"
        :alt="post.author_display_name || post.author_username"
        class="avatar-image"
      />
      <div v-else class="avatar-placeholder">
        {{ (post.author_display_name || post.author_username).charAt(0).toUpperCase() }}
      </div>
    </div>

    <!-- Post content -->
    <div class="post-content">
      <!-- Post card -->
      <div class="post-card" @click="toggleExpand">
        <!-- Title row with category icon -->
        <div class="post-header" :style="{ backgroundColor: categoryConfig.color }">
          <h3 class="post-title">{{ post.title }}</h3>
          <span class="category-icon">{{ categoryConfig.icon }}</span>
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
  align-items: center;
}

.post-date {
  flex-shrink: 0;
  width: 120px;
  text-align: right;
}

.date-absolute {
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.875rem;
  line-height: 1.2;
}

.date-relative {
  color: var(--text-secondary);
  font-size: 0.75rem;
  margin-top: 2px;
  line-height: 1.2;
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
  overflow: hidden;
  background: var(--accent-blue);
}

.avatar-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.avatar-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: 600;
  font-size: 1.125rem;
}

.post-content {
  flex: 1;
  min-width: 0;
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

.post-header {
  padding: var(--spacing-sm) var(--spacing-md);
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--spacing-md);
}

.post-title {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.category-icon {
  font-size: 1rem;
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%);
  color: var(--text-primary);
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
