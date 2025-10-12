<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useTimelineStore } from '@/stores/timeline'
import { useUserStore } from '@/stores/user'
import TimelinePost from '@/components/TimelinePost.vue'
import UserGroupPanel from '@/components/UserGroupPanel.vue'

const route = useRoute()
const timelineStore = useTimelineStore()
const userStore = useUserStore()

const sentinel = ref<HTMLElement | null>(null)
const observer = ref<IntersectionObserver | null>(null)
const userNotFound = ref(false)

// Get username from route parameter
const usernameFromRoute = computed(() => {
  const username = route.params.username
  return typeof username === 'string' ? username : null
})

// Only show sidebar if logged-in user matches the URL user
const showSidebar = computed(() => {
  if (!userStore.isLoggedIn || !usernameFromRoute.value) {
    return false
  }
  return userStore.currentUser?.username === usernameFromRoute.value
})

// Check if viewing another user's timeline
const isViewingOtherUser = computed(() => {
  const routeUsername = usernameFromRoute.value
  const loggedInUsername = userStore.currentUser?.username
  return routeUsername && routeUsername !== loggedInUsername
})

// Setup infinite scroll observer
function setupInfiniteScroll() {
  if (!sentinel.value) return

  // Clean up existing observer first
  if (observer.value) {
    observer.value.disconnect()
  }

  observer.value = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting && timelineStore.hasMore && !timelineStore.loading) {
          timelineStore.loadMore()
        }
      })
    },
    {
      root: null, // viewport
      rootMargin: '500px', // Load much earlier - 500px before reaching the sentinel
      threshold: 0 // Trigger as soon as any part is visible
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
watch(
  [usernameFromRoute, () => userStore.currentUser?.username],
  ([newUsername, loggedInUsername]) => {
    userNotFound.value = false
    // Only apply user filter if viewing someone else's timeline
    if (newUsername && newUsername !== loggedInUsername) {
      timelineStore.setFilter({ userId: newUsername })
    } else if (newUsername) {
      // Viewing own timeline, allow group filtering
      timelineStore.setFilter({})
    } else {
      timelineStore.setFilter({})
    }
  }
)

// Watch for error indicating user not found
watch(
  [() => timelineStore.error, () => timelineStore.posts.length, isViewingOtherUser],
  ([error, postsCount, viewingOther]) => {
    // If viewing another user and got an error or no posts after loading, user might not exist
    if (viewingOther && !timelineStore.loading && postsCount === 0 && error) {
      // Check if error indicates user not found (404 or "not found" message)
      if (error.includes('404') || error.toLowerCase().includes('not found') || error.toLowerCase().includes('user')) {
        userNotFound.value = true
      }
    }
  }
)

// Watch for posts changes to re-setup observer if needed
watch(
  () => timelineStore.posts.length,
  (newLength) => {
    if (newLength > 0 && sentinel.value) {
      // Re-setup observer whenever posts change
      // Use nextTick to ensure DOM is updated
      setTimeout(() => {
        setupInfiniteScroll()
      }, 50)
    }
  }
)

onMounted(() => {
  // Set initial filter from URL
  const routeUsername = usernameFromRoute.value
  const loggedInUsername = userStore.currentUser?.username

  if (routeUsername && routeUsername !== loggedInUsername) {
    // Viewing someone else's timeline, filter by that user
    timelineStore.setFilter({ userId: routeUsername })
  } else {
    // Viewing own timeline or no user specified, allow group filtering
    timelineStore.fetchPosts(true)
  }

  // Setup infinite scroll with a delay to ensure DOM is ready
  setTimeout(() => {
    setupInfiniteScroll()
  }, 200)
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

        <!-- User not found state -->
        <div v-if="userNotFound" class="user-not-found">
          <div class="icon">👤</div>
          <h2>User Not Found</h2>
          <p>The user "{{ usernameFromRoute }}" does not exist or their timeline is not available.</p>
          <p class="hint">Please check the URL and try again.</p>
        </div>

        <!-- Loading state -->
        <div v-else-if="timelineStore.loading && timelineStore.posts.length === 0" class="loading">
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
  gap: var(--spacing-md);
}

.timeline-post-item {
  margin-bottom: 0;
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

.user-not-found {
  text-align: center;
  padding: var(--spacing-xl) var(--spacing-lg);
  max-width: 600px;
  margin: 0 auto;
}

.user-not-found .icon {
  font-size: 4rem;
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.user-not-found h2 {
  color: var(--text-primary);
  margin-bottom: var(--spacing-md);
  font-size: 1.75rem;
}

.user-not-found p {
  color: var(--text-secondary);
  margin-bottom: var(--spacing-sm);
  font-size: 1.125rem;
  line-height: 1.6;
}

.user-not-found .hint {
  font-size: 1rem;
  opacity: 0.8;
  font-style: italic;
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
  width: 100%;
  pointer-events: none;
  visibility: hidden;
  margin: 0;
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
