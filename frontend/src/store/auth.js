import { defineStore } from 'pinia';
import axios from 'axios';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: JSON.parse(localStorage.getItem('user')) || null,
    token: localStorage.getItem('token') || null,
  }),
  getters: {
    isAuthenticated: (state) => !!state.token,
    userRole: (state) => state.user?.role,
  },
  actions: {
    async login(credentials) {
        try {
            const response = await axios.post('/api/auth/login', credentials);
            console.log('Received response from backend:', response.data); // Log the response
            const { user, token } = response.data.data;
            this.user = user;
            this.token = token;
            localStorage.setItem('user', JSON.stringify(user));
            localStorage.setItem('token', token);
        } catch (error) {
            console.error('Error logging in:', error);
            throw error;
        }
    },
    logout() {
      this.user = null;
      this.token = null;
      localStorage.removeItem('user');
      localStorage.removeItem('token');
    },
  },
});
