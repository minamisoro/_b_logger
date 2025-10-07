import { defineStore } from 'pinia'
import { ref } from 'vue'
import openapi from 'openapi-fetch'
import type { components, paths } from 'blogger-lib/api'

const client = openapi<paths>({
  baseUrl: import.meta.env.VITE_API_URL || 'http://localhost:8080'
})

type UserGroupResponse = components['schemas']['UserGroupResponse']
type CreateUserGroupRequest = components['schemas']['CreateUserGroupRequest']
type UpdateUserGroupRequest = components['schemas']['UpdateUserGroupRequest']

export const useUserGroupsStore = defineStore('user-groups', () => {
  const groups = ref<UserGroupResponse[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const selectedGroupId = ref<string | null>(null)

  async function fetchGroups() {
    loading.value = true
    error.value = null

    try {
      const { data, error: fetchError } = await client.GET('/api/user-groups')

      if (fetchError) {
        error.value = fetchError.message || 'Failed to fetch user groups'
        return
      }

      if (data) {
        groups.value = data.groups
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
    } finally {
      loading.value = false
    }
  }

  async function createGroup(request: CreateUserGroupRequest) {
    loading.value = true
    error.value = null

    try {
      const { data, error: createError } = await client.POST('/api/user-groups', {
        body: request
      })

      if (createError) {
        error.value = createError.message || 'Failed to create group'
        return null
      }

      if (data) {
        groups.value.push(data)
        return data
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
      return null
    } finally {
      loading.value = false
    }
  }

  async function updateGroup(id: string, request: UpdateUserGroupRequest) {
    loading.value = true
    error.value = null

    try {
      const { data, error: updateError } = await client.PUT('/api/user-groups/{id}', {
        params: { path: { id } },
        body: request
      })

      if (updateError) {
        error.value = updateError.message || 'Failed to update group'
        return null
      }

      if (data) {
        const index = groups.value.findIndex((g) => g.id === id)
        if (index !== -1) {
          groups.value[index] = data
        }
        return data
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
      return null
    } finally {
      loading.value = false
    }
  }

  async function deleteGroup(id: string) {
    loading.value = true
    error.value = null

    try {
      const { error: deleteError } = await client.DELETE('/api/user-groups/{id}', {
        params: { path: { id } }
      })

      if (deleteError) {
        error.value = deleteError.message || 'Failed to delete group'
        return false
      }

      groups.value = groups.value.filter((g) => g.id !== id)
      if (selectedGroupId.value === id) {
        selectedGroupId.value = null
      }
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
      return false
    } finally {
      loading.value = false
    }
  }

  async function addMember(groupId: string, userId: string) {
    loading.value = true
    error.value = null

    try {
      const { error: addError } = await client.POST('/api/user-groups/{id}/members', {
        params: { path: { id: groupId } },
        body: { user_id: userId }
      })

      if (addError) {
        error.value = addError.message || 'Failed to add member'
        return false
      }

      // Refresh groups to get updated member list
      await fetchGroups()
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
      return false
    } finally {
      loading.value = false
    }
  }

  async function removeMember(groupId: string, userId: string) {
    loading.value = true
    error.value = null

    try {
      const { error: removeError } = await client.DELETE(
        '/api/user-groups/{id}/members/{user_id}',
        {
          params: { path: { id: groupId, user_id: userId } }
        }
      )

      if (removeError) {
        error.value = removeError.message || 'Failed to remove member'
        return false
      }

      // Refresh groups to get updated member list
      await fetchGroups()
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error'
      return false
    } finally {
      loading.value = false
    }
  }

  function selectGroup(groupId: string | null) {
    selectedGroupId.value = groupId
  }

  return {
    groups,
    loading,
    error,
    selectedGroupId,
    fetchGroups,
    createGroup,
    updateGroup,
    deleteGroup,
    addMember,
    removeMember,
    selectGroup
  }
})
