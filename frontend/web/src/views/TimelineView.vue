<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useTimelineStore } from '@/stores/timeline'
import TimelinePost from '@/components/TimelinePost.vue'
import UserGroupPanel from '@/components/UserGroupPanel.vue'

const route = useRoute()
const timelineStore = useTimelineStore()

const sentinel = ref<HTMLElement | null>(null)
const observer = ref<IntersectionObserver | null>(null)

// Check if user is specified in URL
const userIdFromRoute = computed(() => {
  const userId = route.query.user
  return typeof userId === 'string' ? userId : null
})

const showSidebar = computed(() => !userIdFromRoute.value)

// Setup infinite scroll observer
function setupInfiniteScroll() {
  if (!sentinel.value) return

  observer.value = new IntersectionObserver(
    (entries) => {
      const entry = entries[0]
      if (entry && entry.isIntersecting && timelineStore.hasMore && !timelineStore.loading) {
        timelineStore.loadMore()
      }
    },
    {
      threshold: 0.1,
      rootMargin: '100px'
    }
  )

  observer.value.observe(sentinel.value)
}

function cleanupInfiniteScroll() {
  if (observer.value) {
    observer.value.disconnect()
    observer.value = null
  }
}

// Watch for route changes to update filter
watch(userIdFromRoute, (newUserId) => {
  if (newUserId) {
    timelineStore.setFilter({ userId: newUserId })
  } else {
    timelineStore.setFilter({})
  }
})

onMounted(() => {
  // Set initial filter from URL
  if (userIdFromRoute.value) {
    timelineStore.setFilter({ userId: userIdFromRoute.value })
  } else {
    timelineStore.fetchPosts(true)
  }

  // Setup infinite scroll
  setupInfiniteScroll()
})

onUnmounted(() => {
  cleanupInfiniteScroll()
})
</script>

<template>
  <div class="timeline-view">
    <div class="container">
      <!-- Left sidebar: User group filter panel (only shown when no user filter in URL) -->
      <UserGroupPanel v-if="showSidebar" class="sidebar" />

      <!-- Main timeline -->
      <div class="timeline-main">
        <h1>Timeline</h1>

        <!-- Loading state -->
        <div v-if="timelineStore.loading && timelineStore.posts.length === 0" class="loading">
          Loading timeline...
        </div>

        <!-- Error state -->
        <div v-else-if="timelineStore.error" class="error">
          {{ timelineStore.error }}
        </div>

        <!-- Timeline posts -->
        <div v-else class="timeline-container">
          <!-- Vertical timeline line -->
          <div class="timeline-line"></div>

          <!-- Posts -->
          <div class="timeline-posts">
            <TimelinePost
              v-for="post in timelineStore.posts"
              :key="post.id"
              :post="post"
              class="timeline-post-item"
            />

            <!-- Empty state -->
            <div v-if="timelineStore.posts.length === 0" class="empty-state">
              <p>No posts to display.</p>
            </div>
          </div>

          <!-- Loading more indicator -->
          <div v-if="timelineStore.loading && timelineStore.posts.length > 0" class="loading-more">
            Loading more posts...
          </div>

          <!-- Infinite scroll sentinel -->
          <div ref="sentinel" class="sentinel"></div>

          <!-- End of timeline -->
          <div
            v-if="!timelineStore.hasMore && timelineStore.posts.length > 0"
            class="end-of-timeline"
          >
            You've reached the end of the timeline
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.timeline-view {
  padding: var(--spacing-lg);
  min-height: 100vh;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  gap: var(--spacing-xl);
}

.sidebar {
  flex-shrink: 0;
}

.timeline-main {
  flex: 1;
  min-width: 0;
}

h1 {
  color: var(--text-primary);
  margin-bottom: var(--spacing-xl);
  font-size: 2rem;
}

.timeline-container {
  position: relative;
}

.timeline-line {
  position: absolute;
  left: 20px;
  top: 40px;
  bottom: 0;
  width: 2px;
  background: var(--color-border);
  z-index: 1;
}

.timeline-posts {
  position: relative;
  z-index: 2;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
}

.timeline-post-item {
  margin-bottom: var(--spacing-md);
}

.loading,
.error,
.empty-state,
.loading-more,
.end-of-timeline {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--text-secondary);
}

.error {
  color: #d32f2f;
}

.loading-more {
  font-style: italic;
}

.end-of-timeline {
  border-top: 2px solid var(--color-border);
  margin-top: var(--spacing-xl);
  padding-top: var(--spacing-xl);
  opacity: 0.6;
}

.sentinel {
  height: 1px;
  margin-top: var(--spacing-lg);
}

.empty-state p {
  font-size: 1.125rem;
}

@media (max-width: 768px) {
  .container {
    flex-direction: column;
  }

  .sidebar {
    position: static;
    width: 100%;
  }
}
</style>
