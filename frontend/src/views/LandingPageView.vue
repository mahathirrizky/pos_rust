<script setup>
import { computed } from 'vue';
import { useAuthStore } from '../store/auth';

const authStore = useAuthStore();

// This object defines the display properties for each role.
const roleDashboards = {
  Cashier: { dashboardPath: '/cashier', icon: 'pi pi-shopping-cart', title: 'Cashier App', description: 'Process sales and manage orders.' },
  InventoryManager: { dashboardPath: '/inventory-manager', icon: 'pi pi-box', title: 'Inventory Manager', description: 'Manage stock and suppliers.' },
  StoreManager: { dashboardPath: '/store-manager', icon: 'pi pi-users', title: 'Store Manager', description: 'Oversee store operations.' },
  Admin: { dashboardPath: '/admin', icon: 'pi pi-cog', title: 'Admin Panel', description: 'Manage system settings.' },
  Owner: { dashboardPath: '/owner', icon: 'pi pi-briefcase', title: 'Owner Panel', description: 'Oversee business performance.' }
};

// A computed property to get the current user's dashboard info
const userDashboard = computed(() => {
  if (authStore.user && authStore.user.role) {
    return roleDashboards[authStore.user.role];
  }
  return null;
});

const handleLogout = () => {
  authStore.logout();
  // No need to push, the view will update automatically.
};
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4">
    <h1 class="text-4xl font-bold text-gray-800 mb-8">Welcome to the POS System</h1>

    <!-- Authenticated View -->
    <div v-if="authStore.isAuthenticated && userDashboard" class="text-center">
      <p class="text-xl text-gray-600 mb-8">Hello, {{ authStore.user.first_name }}!</p>
      <router-link :to="userDashboard.dashboardPath" class="block">
        <div class="bg-white rounded-lg shadow-lg hover:shadow-xl transition-shadow duration-300 p-8 text-center cursor-pointer max-w-sm mx-auto">
          <i :class="userDashboard.icon" class="text-6xl text-primary mb-4"></i>
          <h2 class="text-2xl font-semibold text-gray-700 mb-2">Go to {{ userDashboard.title }}</h2>
          <p class="text-gray-600">{{ userDashboard.description }}</p>
        </div>
      </router-link>
      <button @click="handleLogout" class="px-6 py-2 mt-8 text-white bg-red-600 rounded-lg hover:bg-red-800 transition-colors">
        Logout
      </button>
    </div>

    <!-- Unauthenticated View -->
    <div v-else>
      <p class="text-xl text-gray-600 mb-12 text-center">Please select your application to log in.</p>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <div v-for="(dashboard, role) in roleDashboards" :key="role">
          <!-- The link now points to the protected dashboard path -->
          <router-link :to="dashboard.dashboardPath" class="block bg-white rounded-lg shadow-lg hover:shadow-xl transition-shadow duration-300 p-8 text-center cursor-pointer h-full">
            <i :class="dashboard.icon" class="text-6xl text-primary mb-4"></i>
            <h2 class="text-2xl font-semibold text-gray-700 mb-2">{{ dashboard.title }}</h2>
            <p class="text-gray-600">{{ dashboard.description }}</p>
          </router-link>
        </div>
      </div>
    </div>

  </div>
</template>

<style scoped>
.text-primary {
  color: #4F46E5;
}
</style>