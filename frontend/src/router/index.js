import { createRouter, createWebHistory } from 'vue-router';
import cashierRoutes from './routes/cashier.js';
import inventoryManagerRoutes from './routes/inventoryManager.js';
import LandingPageView from '../views/LandingPageView.vue';
import LoginView from '../views/auth/LoginView.vue';
import UnauthorizedView from '../views/auth/UnauthorizedView.vue';

const routes = [
  { path: '/', name: 'LandingPage', component: LandingPageView },
  { path: '/login', name: 'Login', component: LoginView },
  { path: '/unauthorized', name: 'Unauthorized', component: UnauthorizedView },
  cashierRoutes,
  inventoryManagerRoutes,
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  // Dummy authentication and role for demonstration
  const isAuthenticated = true; // Replace with actual auth check
  const userRole = 'Admin'; // Replace with actual user role from auth store

  // Routes that do not require authentication
  const publicPages = ['/login', '/', '/unauthorized'];
  const authRequired = !publicPages.includes(to.path);

  if (authRequired && !isAuthenticated) {
    next('/login'); // Redirect to login page if not authenticated
  } else if (to.meta.allowedRoles && !to.meta.allowedRoles.includes(userRole)) {
    next('/unauthorized'); // Redirect to unauthorized page if role not allowed
  } else {
    next(); // Proceed to route
  }
});

export default router;
