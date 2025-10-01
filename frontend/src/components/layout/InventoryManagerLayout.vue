<script setup>
import { computed } from 'vue';
import Sidebar from './Sidebar.vue';
import { useAuthStore } from '../../store/auth';

const authStore = useAuthStore();

const employee = computed(() => authStore.user);
</script>

<template>
  <div class="flex bg-surface-50 dark:bg-surface-900">
    <Sidebar :employee="authStore.user" />
    <main class="flex-grow p-6 h-screen overflow-auto">
      <router-view />
    </main>
    <div v-if="employee" class="absolute bottom-0 right-0 p-4 text-sm text-surface-600 dark:text-surface-300">
      Logged in as: {{ employee.first_name }} {{ employee.last_name }} ({{ employee.role }})
    </div>
  </div>
</template>
