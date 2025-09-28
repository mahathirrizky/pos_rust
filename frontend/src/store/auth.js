import { defineStore } from 'pinia';
import axios from 'axios';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: JSON.parse(localStorage.getItem('user')) || null,
    token: localStorage.getItem('token') || null,
    permissions: JSON.parse(localStorage.getItem('permissions')) || [],
  }),
  getters: {
    isAuthenticated: (state) => !!state.token,
    userRole: (state) => state.user?.role,
    hasPermission: (state) => (permission) => state.permissions.includes(permission),
  },
  actions: {
    async login(credentials) {
        try {
            const response = await axios.post('/api/auth/login', credentials);
            console.log('Received response from backend:', response.data); // Log the response
            const { user, token, permissions } = response.data.data;
            this.user = user;
            this.token = token;
            this.permissions = permissions || [];
            localStorage.setItem('user', JSON.stringify(user));
            localStorage.setItem('token', token);
            localStorage.setItem('permissions', JSON.stringify(permissions || []));
        } catch (error) {
            console.error('Error logging in:', error);
            throw error;
        }
    },
    logout() {
      this.user = null;
      this.token = null;
      this.permissions = [];
      localStorage.removeItem('user');
      localStorage.removeItem('token');
      localStorage.removeItem('permissions');
    },
  },
});
