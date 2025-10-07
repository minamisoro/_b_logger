<script setup lang="ts">
import { onMounted } from 'vue'
import { useUserGroupsStore } from '@/stores/user-groups'
import { useTimelineStore } from '@/stores/timeline'

const userGroupsStore = useUserGroupsStore()
const timelineStore = useTimelineStore()

onMounted(() => {
  userGroupsStore.fetchGroups()
})

function selectAll() {
  userGroupsStore.selectGroup(null)
  timelineStore.setFilter({})
}

function selectGroup(groupId: string) {
  userGroupsStore.selectGroup(groupId)
  timelineStore.setFilter({ groupId })
}
</script>

<template>
  <div class="user-group-panel">
    <h2>Filter Timeline</h2>

    <div v-if="userGroupsStore.loading" class="loading">Loading groups...</div>
    <div v-else-if="userGroupsStore.error" class="error">
      {{ userGroupsStore.error }}
    </div>
    <div v-else class="groups-list">
      <!-- All posts option -->
      <div
        class="group-item"
        :class="{ active: userGroupsStore.selectedGroupId === null }"
        @click="selectAll"
      >
        <div class="group-icon">🌐</div>
        <div class="group-info">
          <div class="group-name">All Posts</div>
          <div class="group-description">Show posts from all users</div>
        </div>
      </div>

      <!-- User groups -->
      <div
        v-for="group in userGroupsStore.groups"
        :key="group.id"
        class="group-item"
        :class="{ active: userGroupsStore.selectedGroupId === group.id }"
        @click="selectGroup(group.id)"
      >
        <div class="group-icon">👥</div>
        <div class="group-info">
          <div class="group-name">{{ group.name }}</div>
          <div class="group-description">{{ group.members.length }} members</div>
        </div>
      </div>

      <div v-if="userGroupsStore.groups.length === 0" class="empty-state">
        <p>No user groups yet.</p>
        <p class="hint">Create groups to organize and filter posts by multiple users.</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.user-group-panel {
  width: 280px;
  background: var(--color-background-soft);
  border-radius: 8px;
  padding: var(--spacing-lg);
  height: fit-content;
  position: sticky;
  top: var(--spacing-lg);
}

h2 {
  margin: 0 0 var(--spacing-md) 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.loading,
.error {
  padding: var(--spacing-md);
  text-align: center;
  color: var(--text-secondary);
}

.error {
  color: #d32f2f;
}

.groups-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.group-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.group-item:hover {
  background: var(--color-background-mute);
}

.group-item.active {
  background: var(--color-background-mute);
  border: 2px solid var(--color-border-hover);
}

.group-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.group-info {
  flex: 1;
  min-width: 0;
}

.group-name {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.group-description {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.empty-state {
  padding: var(--spacing-md);
  text-align: center;
  color: var(--text-secondary);
}

.empty-state p {
  margin: var(--spacing-sm) 0;
}

.hint {
  font-size: 0.875rem;
  opacity: 0.8;
}
</style>
