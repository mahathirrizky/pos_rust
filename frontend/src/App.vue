<script setup>
import Toast from 'primevue/toast';
import ConfirmDialog from 'primevue/confirmdialog';
</script>

<template>
  <Toast />
  <ConfirmDialog />
  <router-view />
</template>

<style>
/* Global styles can go here if needed */
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol';
}
</style>
