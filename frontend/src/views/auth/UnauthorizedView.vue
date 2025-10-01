<template>
  <div class="flex items-center justify-center min-h-screen bg-surface-100 dark:bg-surface-900">
    <div class="px-8 py-6 mt-4 text-left bg-surface-0 dark:bg-surface-800 shadow-lg">
      <h3 class="text-2xl font-bold text-center text-red-600">Unauthorized Access</h3>
      <p class="mt-4 text-surface-700 dark:text-surface-200">You do not have permission to view this page.</p>
      <div class="flex justify-center mt-6">
        <router-link to="/" class="px-6 py-2 text-white bg-blue-600 rounded-lg hover:bg-blue-900">Go to Home</router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
// Script for Unauthorized View
</script>

<style scoped>
/* Scoped styles for this component */
</style>