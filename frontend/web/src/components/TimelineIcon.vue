<script setup lang="ts">
const { type } = defineProps<{
  type: 'generic'
}>();

</script>

<template>
  <v-icon v-if="type === 'generic'" icon="mdi-plus" size="small" color="white-2"/>
</template>

<style scoped></style>