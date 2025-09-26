<script setup>
import { computed } from 'vue';
import Sidebar from './Sidebar.vue';
import { useAuthStore } from '../../store/auth';

const authStore = useAuthStore();

const employee = computed(() => authStore.user);
</script>

<template>
    <div class="flex h-screen bg-gray-50">
      <Sidebar :employee="authStore.user" />
      <div class="flex-1 flex flex-col overflow-hidden">
        <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-200 p-4">
          <router-view />
        </main>
      </div>
    </div>
  </template>
