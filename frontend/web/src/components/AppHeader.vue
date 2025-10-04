<template>
  <v-app-bar color="primary" elevation="2">
    <v-container fluid class="d-flex align-center px-4">
      <div class="d-flex align-center">
        <v-btn :to="{ name: 'timeline' }" variant="text" color="white" size="large">
          <v-icon start>mdi-home</v-icon>
          _B_logger
        </v-btn>
      </div>

      <v-spacer></v-spacer>

      <div class="d-flex align-center">
        <v-btn to="/articles" variant="text" color="white">
          Articles
        </v-btn>
        <v-btn to="/categories" variant="text" color="white">
          Categories
        </v-btn>
        <v-btn to="/about" variant="text" color="white">
          About
        </v-btn>
      </div>
    </v-container>
  </v-app-bar>
</template>

<script setup lang="ts">
// Header component for the blog
// Links will be implemented at a later date
</script>

<style scoped>
.v-app-bar {
  position: sticky;
  top: 0;
  z-index: 1000;
}
</style>
