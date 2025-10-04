<script setup lang="ts">
import { ref } from 'vue'
import TimelineView from '@/components/TimelineIcon.vue'

const posts = ref([
  {
    id: 1,
    title: 'Welcome to _B_logger',
    date: '2024-10-04',
    content: 'This is the beginning of our journey...',
  }
])
</script>

<template>
  <v-container>
    <v-row>
      <v-col cols="12" md="8" offset-md="2">
        <v-timeline align="start">
          <v-timeline-item
            v-for="post in posts"
            :key="post.id"
            dot-color="primary"
            size="small"
            density="compact"
          >
          <template v-slot:icon>
            <timeline-view type="generic" />
          </template>
          <v-card>
            <template v-slot:title>
              <v-btn >{{ post.title }}</v-btn>
            </template>
            <template v-slot:text>
              {{ post.content }}
            </template>
          </v-card>
          </v-timeline-item>
        </v-timeline>
      </v-col>
    </v-row>
  </v-container>
</template>
