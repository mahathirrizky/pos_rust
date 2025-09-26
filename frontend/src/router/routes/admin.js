import AdminLayout from '../../components/layout/AdminLayout.vue';

const adminRoutes = [
  {
    path: '/admin',
    component: AdminLayout,
    meta: { allowedRoles: ['Admin', 'Owner'] },
    children: [
      {
        path: '',
        name: 'admin-dashboard',
        component: () => import('../../views/admin/Dashboard.vue'),

      },
      {
        path: 'stores',
        name: 'admin-stores',
        component: () => import('../../views/admin/Stores.vue'),

      },
      {
        path: 'customers',
        name: 'admin-customers',
        component: () => import('../../views/admin/Customers.vue'),

      },
      {
        path: 'categories',
        name: 'admin-categories',
        component: () => import('../../views/admin/Categories.vue'),

      },
      {
        path: 'bills',
        name: 'admin-bills',
        component: () => import('../../views/admin/Bills.vue'),

      },
      {
        path: 'settings',
        name: 'admin-settings',
        component: () => import('../../views/admin/Settings.vue'),

      },
      {
        path: 'inventory',
        name: 'admin-inventory',
        component: () => import('../../views/admin/Inventory.vue'),

      },
      {
        path: 'products',
        name: 'admin-products',
        component: () => import('../../views/admin/Products.vue'),

      },
      {
        path: 'suppliers',
        name: 'admin-suppliers',
        component: () => import('../../views/admin/Suppliers.vue'),

      },
      {
        path: 'purchase-orders',
        name: 'admin-purchase-orders',
        component: () => import('../../views/purchase-orders/PurchaseOrderListView.vue'),
      },
      {
        path: 'purchase-orders/new',
        name: 'admin-create-purchase-order',
        component: () => import('../../views/purchase-orders/CreatePurchaseOrderView.vue'),
      },
      {
        path: 'purchase-orders/:id',
        name: 'admin-purchase-order-detail',
        component: () => import('../../views/purchase-orders/PurchaseOrderDetailView.vue'),
      },
      {
        path: 'reports',
        name: 'admin-reports',
        component: () => import('../../views/admin/Reports.vue'),

      },
      {
        path: 'employees',
        name: 'admin-employees',
        component: () => import('../../views/admin/Employees.vue'),

      },
      {
        path: 'promotions',
        name: 'admin-promotions',
        component: () => import('../../views/admin/Promotions.vue'),

      },
      {
        path: 'refunds',
        name: 'admin-refunds',
        component: () => import('../../views/admin/Refunds.vue'),

      },
    ],
  },
];

export default adminRoutes;
