<script setup>
import { computed } from 'vue';
import Sidebar from './Sidebar.vue';
import { useAuthStore } from '../../store/auth';

const authStore = useAuthStore();

const employee = computed(() => authStore.user);
</script>

<template>
  <div class="flex h-screen bg-surface-50 dark:bg-surface-900">
    <Sidebar :employee="employee" />
    <div class="flex flex-col flex-grow">
      <main class="flex-grow overflow-hidden p-6">
        <router-view />
      </main>
      <footer v-if="employee" class="p-4 text-sm text-right text-surface-600 dark:text-surface-300 border-t border-surface-200 dark:border-surface-700">
        Logged in as: <strong>{{ employee.first_name }} {{ employee.last_name }}</strong> ({{ employee.role }})
      </footer>
    </div>
  </div>
</template>
