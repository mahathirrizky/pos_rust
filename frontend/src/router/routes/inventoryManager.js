import InventoryManagerLayout from '../../components/layout/InventoryManagerLayout.vue';

const inventoryManagerRoutes = {
  path: '/inventory-manager',
  component: InventoryManagerLayout,
  meta: { allowedRoles: ['InventoryManager', 'Admin', 'Owner'] },
  children: [
    {
      path: '',
      name: 'InventoryDashboard',
      component: () => import('../../views/inventoryManager/InventoryDashboardView.vue'),
      
    },
    {
      path: 'inventory',
      name: 'InventoryList',
      component: () => import('../../views/inventoryManager/Inventory.vue'),
      
    },
    {
      path: 'products',
      name: 'ProductManagement',
      component: () => import('../../views/inventoryManager/Products.vue'),
      
    },
    {
      path: 'suppliers',
      name: 'SupplierManagement',
      component: () => import('../../views/inventoryManager/Suppliers.vue'),
      
    },
    {
      path: 'purchase-orders',
      name: 'PurchaseOrderList',
      component: () => import('../../views/purchase-orders/PurchaseOrderListView.vue'),
    },
    {
      path: 'purchase-orders/new',
      name: 'CreatePurchaseOrder',
      component: () => import('../../views/purchase-orders/CreatePurchaseOrderView.vue'),
    },
    {
      path: 'purchase-orders/:id',
      name: 'PurchaseOrderDetail',
      component: () => import('../../views/purchase-orders/PurchaseOrderDetailView.vue'),
    },
    {
      path: 'reports',
      name: 'InventoryReports',
      component: () => import('../../views/inventoryManager/Reports.vue'),
      
    },
    {
      path: 'settings',
      name: 'InventorySettings',
      component: () => import('../../views/inventoryManager/Settings.vue'),
      
    },
  ],}
export default inventoryManagerRoutes