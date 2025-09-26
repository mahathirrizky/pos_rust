<script setup>
import { computed } from 'vue';
import Sidebar from './Sidebar.vue';
import { useAuthStore } from '../../store/auth';

const authStore = useAuthStore();

const employee = computed(() => authStore.user);
</script>

<template>
  <div class="flex bg-gray-50">
    <Sidebar :employee="employee" />
    <main class="flex-grow p-6 h-screen overflow-auto">
      <router-view />
    </main>
    <div v-if="employee" class="absolute bottom-0 right-0 p-4 text-sm text-gray-600">
      Logged in as: {{ employee.first_name }} {{ employee.last_name }} ({{ employee.role }})
    </div>
  </div>
</template>
