<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  employee: Object // Define employee prop
});

const menu = ref([
  // Cashier/General User Links
  { to: '/cashier', icon: 'pi pi-home', tooltip: 'Home', roles: ['Cashier', 'Admin', 'StoreManager'] },
  { to: '/cashier/bills', icon: 'pi pi-file', tooltip: 'Bills', roles: ['Cashier', 'Admin', 'StoreManager'] },
  { to: '/cashier/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['Cashier', 'Admin', 'StoreManager'] },

  // Inventory Manager Links
  { to: '/inventory-manager', icon: 'pi pi-chart-line', tooltip: 'Dashboard', roles: ['Admin', 'StoreManager', 'InventoryManager'] }, // Consolidated Home/Dashboard
  { to: '/inventory-manager/inventory', icon: 'pi pi-box', tooltip: 'Inventory', roles: ['Admin', 'StoreManager', 'InventoryManager'] }, // Specific Inventory List
  { to: '/inventory-manager/products', icon: 'pi pi-tag', tooltip: 'Products', roles: ['Admin', 'StoreManager', 'InventoryManager'] },
  { to: '/inventory-manager/suppliers', icon: 'pi pi-truck', tooltip: 'Suppliers', roles: ['Admin', 'StoreManager', 'InventoryManager'] },
  { to: '/inventory-manager/reports', icon: 'pi pi-chart-bar', tooltip: 'Reports', roles: ['Admin', 'StoreManager', 'InventoryManager'] },
  { to: '/inventory-manager/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['Admin', 'StoreManager', 'InventoryManager'] },

  // Global Links (if any, e.g., Landing Page)
  { to: '/', icon: 'pi pi-globe', tooltip: 'Landing Page', roles: ['Admin'] }, // Only Admin sees this, or make it public if needed
]);

const filteredMenu = computed(() => {
  if (!props.employee || !props.employee.role) {
    return [];
  }
  return menu.value.filter(item => item.roles.includes(props.employee.role));
});

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
      <li v-for="item in filteredMenu" :key="item.to" class="my-4">
        <router-link :to="item.to" v-tooltip.right="item.tooltip"
          class="flex items-center justify-center p-3 rounded-lg text-gray-600 hover:bg-primary-100 hover:text-primary transition-colors duration-200"
          active-class="bg-primary-500 text-white"
          exact-active-class="bg-primary-500 text-white">
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
