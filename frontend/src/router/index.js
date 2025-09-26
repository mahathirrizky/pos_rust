import { createRouter, createWebHistory } from 'vue-router';
import { useAuthStore } from '../store/auth';
import cashierRoutes from './routes/cashier.js';
import inventoryManagerRoutes from './routes/inventoryManager.js';
import storeManagerRoutes from './routes/storeManager.js';
import adminRoutes from './routes/admin.js';
import ownerRoutes from './routes/owner.js';

const loginRoutes = [
  { path: '/admin/login', name: 'AdminLogin', component: () => import('../views/auth/LoginView.vue') },
  { path: '/owner/login', name: 'OwnerLogin', component: () => import('../views/auth/LoginView.vue') },
  { path: '/store-manager/login', name: 'StoreManagerLogin', component: () => import('../views/auth/LoginView.vue') },
  { path: '/inventory-manager/login', name: 'InventoryManagerLogin', component: () => import('../views/auth/LoginView.vue') },
  { path: '/cashier/login', name: 'CashierLogin', component: () => import('../views/auth/LoginView.vue') },
];

const routes = [
  { path: '/', name: 'LandingPage', component: () => import('../views/LandingPageView.vue') },
  { path: '/unauthorized', name: 'Unauthorized', component: () => import('../views/auth/UnauthorizedView.vue') },
  ...loginRoutes,
  cashierRoutes,
  inventoryManagerRoutes,
  ...storeManagerRoutes,
  ...adminRoutes,
  ...ownerRoutes,
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  const auth = useAuthStore();
  const isLoginPage = to.path.endsWith('/login');
  const hasValidSession = auth.isAuthenticated && auth.userRole;

  // If the route has meta fields, it's a protected route.
  if (to.meta.allowedRoles) {
    if (hasValidSession) {
      // User is logged in, check if they have the correct role.
      if (to.meta.allowedRoles.includes(auth.userRole)) {
        return next(); // Success, proceed.
      } else {
        // User does not have the right role.
        return next('/unauthorized');
      }
    } else {
      // User is not logged in or has an invalid session, redirect to the specific login page for that route.
      return next(to.path + '/login');
    }
  }

  // For public pages (like '/', '/admin/login', etc.)
  // If user is already logged in and tries to access a login page, redirect them.
  if (hasValidSession && isLoginPage) {
    return next('/');
  }

  // Otherwise, allow navigation.
  return next();
});

export default router;