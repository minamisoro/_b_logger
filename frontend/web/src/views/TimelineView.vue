<script setup lang="ts">
import { ref, onMounted } from 'vue'
import TimelineIcon from '@/components/TimelineIcon.vue'
import { apiClient } from '@/api/client'
import type { TimelinePost } from '@lib/client'

const posts = ref<TimelinePost[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    loading.value = true
    error.value = null

    const { data, error: apiError } = await apiClient.timeline.getTimeline({
      limit: 10,
      pages: 0
    })

    if (data) {
      posts.value = data.posts
    } else if (apiError) {
      error.value = 'Failed to load timeline. Please try again later.'
      console.error('Failed to load timeline:', apiError)
    }
  } catch (err) {
    error.value = 'An unexpected error occurred while loading the timeline.'
    console.error('Unexpected error:', err)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <v-container>
    <v-row>
      <v-col cols="12" md="8" offset-md="2">
        <!-- Loading State -->
        <div v-if="loading" class="d-flex justify-center align-center" style="min-height: 400px">
          <v-progress-circular
            indeterminate
            color="primary"
            size="64"
          />
        </div>

        <!-- Error State -->
        <v-alert
          v-else-if="error"
          type="error"
          variant="tonal"
          closable
          @click:close="error = null"
        >
          {{ error }}
        </v-alert>

        <!-- Timeline Content -->
        <v-timeline v-else-if="posts.length > 0" align="start">
          <v-timeline-item
            v-for="post in posts"
            :key="post.id"
            dot-color="primary"
            size="small"
            density="compact"
          >
            <template v-slot:icon>
              <timeline-icon type="generic" />
            </template>
            <v-card>
              <template v-slot:title>
                <v-btn>{{ post.title }}</v-btn>
              </template>
              <template v-slot:text>
                {{ post.content }}
              </template>
            </v-card>
          </v-timeline-item>
        </v-timeline>

        <!-- Empty State -->
        <v-card v-else variant="outlined">
          <v-card-text class="text-center py-8">
            <v-icon size="64" color="grey-lighten-1" class="mb-4">
              mdi-timeline-text-outline
            </v-icon>
            <div class="text-h6 text-grey">No posts available</div>
            <div class="text-body-2 text-grey-lighten-1 mt-2">
              Check back later for new content
            </div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>
