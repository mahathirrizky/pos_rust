<script setup>
import { ref } from 'vue';

const menu = ref([
  { to: '/cashier', icon: 'pi pi-home', tooltip: 'Home' },
  { to: '/bills', icon: 'pi pi-file', tooltip: 'Bills' },
  { to: '/settings', icon: 'pi pi-cog', tooltip: 'Settings' },
]);

const handleLogout = () => {
  // Handle logout logic
  alert('Logout clicked!');
};
</script>

<template>
  <div class="flex flex-col h-screen px-2 py-2 bg-gray-100 shadow-lg w-[90px] flex-shrink-0">
    <!-- Logo or App Name -->
    <div class="flex items-center justify-center mb-6 pt-2">
      <span class="pi pi-prime text-4xl text-primary"></span>
    </div>

    <!-- Main Menu -->
    <ul class="flex-grow">
      <li v-for="item in menu" :key="item.to" class="my-4">
        <router-link :to="item.to" v-tooltip.right="item.tooltip"
          class="flex items-center justify-center p-3 rounded-lg text-gray-600 hover:bg-primary-100 hover:text-primary transition-colors duration-200"
          active-class="bg-primary-500 text-white">
          <i :class="item.icon" class="text-4xl"></i>
        </router-link>
      </li>
    </ul>

    <!-- Logout -->
    <div>
      <button @click="handleLogout" v-tooltip.right="'Logout'"
        class="flex items-center justify-center p-3 rounded-lg text-red-500 hover:bg-red-100 hover:text-red-700 w-full transition-colors duration-200">
        <i class="pi pi-power-off text-4xl"></i>
      </button>
    </div>
  </div>
</template>

<style scoped>
/* Tooltip styling if needed */
</style>
