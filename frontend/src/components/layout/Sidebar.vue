<script setup>
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../../store/auth';

const props = defineProps({
  employee: Object // Define employee prop
});

const router = useRouter();
const authStore = useAuthStore();

const menu = [
  // Cashier/General User Links
  { to: '/cashier', icon: 'pi pi-home', tooltip: 'Home', roles: ['Cashier'] },
  { to: '/cashier/bills', icon: 'pi pi-file', tooltip: 'Bills', roles: ['Cashier'] },
  { to: '/cashier/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['Cashier'] },

  // Inventory Manager Links
  { to: '/inventory-manager', icon: 'pi pi-chart-line', tooltip: 'Dashboard', roles: ['InventoryManager'] }, // Consolidated Home/Dashboard
  { to: '/inventory-manager/inventory', icon: 'pi pi-box', tooltip: 'Inventory', roles: ['InventoryManager'] }, // Specific Inventory List
  { to: '/inventory-manager/products', icon: 'pi pi-tag', tooltip: 'Products', roles: ['InventoryManager'] },
  { to: '/inventory-manager/suppliers', icon: 'pi pi-truck', tooltip: 'Suppliers', roles: ['InventoryManager'] },
  { to: '/inventory-manager/purchase-orders', icon: 'pi pi-shopping-cart', tooltip: 'Purchase Orders', roles: ['InventoryManager'] },
  { to: '/inventory-manager/reports', icon: 'pi pi-chart-bar', tooltip: 'Reports', roles: ['InventoryManager'] },
  { to: '/inventory-manager/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['InventoryManager'] },

  // Store Manager Links
  { to: '/store-manager', icon: 'pi pi-chart-line', tooltip: 'Dashboard', roles: ['StoreManager'] },
  { to: '/store-manager/inventory', icon: 'pi pi-box', tooltip: 'Inventory', roles: ['StoreManager'] },
  { to: '/store-manager/products', icon: 'pi pi-tag', tooltip: 'Products', roles: ['StoreManager'] },
  { to: '/store-manager/suppliers', icon: 'pi pi-truck', tooltip: 'Suppliers', roles: ['StoreManager'] },
  { to: '/store-manager/purchase-orders', icon: 'pi pi-shopping-cart', tooltip: 'Purchase Orders', roles: ['StoreManager'] },
  { to: '/store-manager/reports', icon: 'pi pi-chart-bar', tooltip: 'Reports', roles: ['StoreManager'] },
  { to: '/store-manager/employees', icon: 'pi pi-users', tooltip: 'Employees', roles: ['StoreManager'] },
  { to: '/store-manager/promotions', icon: 'pi pi-gift', tooltip: 'Promotions', roles: ['StoreManager'] },
  { to: '/store-manager/refunds', icon: 'pi pi-replay', tooltip: 'Refunds', roles: ['StoreManager'] },
  { to: '/store-manager/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['StoreManager'] },

  // Admin Links
  { to: '/admin', icon: 'pi pi-shield', tooltip: 'Admin Dashboard', roles: ['Admin'] },
  { to: '/admin/stores', icon: 'pi pi-building', tooltip: 'Stores', roles: ['Admin'] },
  { to: '/admin/customers', icon: 'pi pi-users', tooltip: 'Customers', roles: ['Admin'] },
  { to: '/admin/categories', icon: 'pi pi-sitemap', tooltip: 'Categories', roles: ['Admin'] },
  { to: '/admin/bills', icon: 'pi pi-file', tooltip: 'Bills', roles: ['Admin'] },
  { to: '/admin/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['Admin'] },
  { to: '/admin/inventory', icon: 'pi pi-box', tooltip: 'Inventory', roles: ['Admin'] },
  { to: '/admin/products', icon: 'pi pi-tag', tooltip: 'Products', roles: ['Admin'] },
  { to: '/admin/suppliers', icon: 'pi pi-truck', tooltip: 'Suppliers', roles: ['Admin'] },
  { to: '/admin/purchase-orders', icon: 'pi pi-shopping-cart', tooltip: 'Purchase Orders', roles: ['Admin'] },
  { to: '/admin/reports', icon: 'pi pi-chart-bar', tooltip: 'Reports', roles: ['Admin'] },
  { to: '/admin/employees', icon: 'pi pi-users', tooltip: 'Employees', roles: ['Admin'] },
  { to: '/admin/promotions', icon: 'pi pi-gift', tooltip: 'Promotions', roles: ['Admin'] },
  { to: '/admin/refunds', icon: 'pi pi-replay', tooltip: 'Refunds', roles: ['Admin'] },

  // Owner Links
  { to: '/owner', icon: 'pi pi-briefcase', tooltip: 'Owner Dashboard', roles: ['Owner'] },
  { to: '/owner/admin-users', icon: 'pi pi-user-plus', tooltip: 'Manage Admin Users', roles: ['Owner'] },
  { to: '/owner/employees', icon: 'pi pi-users', tooltip: 'Employees', roles: ['Owner'] },
  { to: '/owner/stores', icon: 'pi pi-building', tooltip: 'Manage Stores', roles: ['Owner'] },
  { to: '/admin/roles', icon: 'pi pi-key', tooltip: 'Role Management', roles: ['Admin', 'Owner'] }, // Moved from Admin section
  { to: '/owner/products', icon: 'pi pi-tag', tooltip: 'Products', roles: ['Owner'] },
  { to: '/owner/inventory', icon: 'pi pi-box', tooltip: 'Inventory', roles: ['Owner'] },
  { to: '/owner/suppliers', icon: 'pi pi-truck', tooltip: 'Suppliers', roles: ['Owner'] },
  { to: '/owner/purchase-orders', icon: 'pi pi-shopping-cart', tooltip: 'Purchase Orders', roles: ['Owner'] },
  { to: '/owner/reports', icon: 'pi pi-chart-bar', tooltip: 'Reports', roles: ['Owner'] },
  { to: '/owner/settings', icon: 'pi pi-cog', tooltip: 'Settings', roles: ['Owner'] },

  
];

const filteredMenu = computed(() => {
  if (!props.employee || !props.employee.role) {
    return [];
  }
  return menu.filter(item => {
    // Ensure item and item.roles are defined before proceeding
    if (!item || !item.roles) {
      return false;
    }
    return item.roles.includes(props.employee.role);
  });
});

const handleLogout = () => {
  authStore.logout();
  router.push('/');
};
</script>

<template>
  <div class="flex flex-col h-screen px-2 py-2 bg-surface-50 dark:bg-surface-800 shadow-lg w-[90px] flex-shrink-0">
    <!-- Logo or App Name -->
    <div class="flex items-center justify-center mb-6 pt-2">
      <span class="pi pi-prime text-3xl text-primary"></span>
    </div>

    <!-- Main Menu -->
    <ul class="flex-grow overflow-y-auto pr-2">
      <li v-for="item in filteredMenu" :key="item.to" class="my-4 flex justify-center">
        <router-link :to="item.to" v-tooltip.right="item.tooltip"
          class="flex items-center justify-center w-16 h-16 rounded-lg text-surface-600 dark:text-surface-300 hover:bg-primary-100 hover:text-primary transition-colors duration-200"
          active-class="bg-primary-500 text-white"
          exact-active-class="bg-primary-500 text-white">
          <i :class="item.icon" class="text-3xl"></i>
        </router-link>
      </li>
    </ul>

    <!-- Logout -->
    <div>
      <button @click="handleLogout" v-tooltip.right="'Logout'"
        class="flex items-center justify-center p-3 rounded-lg text-red-500 hover:bg-red-100 hover:text-red-700 w-full transition-colors duration-200">
        <i class="pi pi-sign-out text-3xl"></i>
      </button>
    </div>
  </div>
</template>
