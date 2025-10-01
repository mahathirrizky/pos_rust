<script setup>
import Toast from 'primevue/toast';
import ConfirmDialog from 'primevue/confirmdialog';
import { useThemeStore } from './store/theme'; // Import the theme store

// Initialize the theme store globally
useThemeStore();

</script>

<template>
  <div class="min-h-screen bg-surface-100 text-surface-800 dark:bg-surface-900 dark:text-surface-100">
    <Toast />
    <ConfirmDialog />
    <router-view />
  </div>
</template>

<style>
/* Global styles can go here if needed */
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol';
}
</style>
