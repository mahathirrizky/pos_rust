import InventoryManagerLayout from '../../components/layout/InventoryManagerLayout.vue';
import InventoryDashboardView from '../../views/inventoryManager/InventoryDashboardView.vue';
import SettingsView from '../../views/placeholder/SettingsView.vue';
import InventoryListView from '../../views/inventoryManager/InventoryListView.vue'; // New import
import ProductManagementView from '../../views/inventoryManager/ProductManagementView.vue'; // New import
import SupplierManagementView from '../../views/inventoryManager/SupplierManagementView.vue'; // New import
import ReportsView from '../../views/inventoryManager/ReportsView.vue'; // New import

const inventoryManagerRoutes = {
  path: '/inventory-manager',
  component: InventoryManagerLayout,
  children: [
    {
      path: '',
      name: 'InventoryDashboard',
      component: InventoryDashboardView,
      meta: { requiresAuth: true, allowedRoles: ['Admin', 'StoreManager', 'InventoryManager'] }
    },
    {
      path: 'inventory', // New route for Inventory List
      name: 'InventoryList',
      component: InventoryListView,
      meta: { requiresAuth: true, allowedRoles: ['Admin', 'StoreManager', 'InventoryManager'] }
    },
    {
      path: 'products', // New route for Product Management
      name: 'ProductManagement',
      component: ProductManagementView,
      meta: { requiresAuth: true, allowedRoles: ['Admin', 'StoreManager', 'InventoryManager'] }
    },
    {
      path: 'suppliers', // New route for Supplier Management
      name: 'SupplierManagement',
      component: SupplierManagementView,
      meta: { requiresAuth: true, allowedRoles: ['Admin', 'StoreManager', 'InventoryManager'] }
    },
    {
      path: 'reports', // New route for Reports
      name: 'InventoryReports',
      component: ReportsView,
      meta: { requiresAuth: true, allowedRoles: ['Admin', 'StoreManager', 'InventoryManager'] }
    },
    {
      path: 'settings',
      name: 'InventorySettings',
      component: SettingsView,
      meta: { requiresAuth: true, allowedRoles: ['Admin', 'StoreManager', 'InventoryManager'] }
    },
  ],}
export default inventoryManagerRoutes