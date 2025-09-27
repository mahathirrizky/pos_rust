import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth'; // Import auth store

export const useCategoryStore = defineStore('category', {
  state: () => ({
    categories: [],
    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchCategories() {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore();
      if (!authStore.token) {
        this.error = new Error('Not authenticated');
        this.isLoading = false;
        return;
      }

      try {
        const response = await axios.get('/api/categories', { // FIX: URL
          headers: { Authorization: `Bearer ${authStore.token}` } // FIX: Header
        });
        this.categories = response.data.data; // FIX: Data nesting
      } catch (error) {
        this.error = error;
        console.error('Error fetching categories:', error);
      } finally {
        this.isLoading = false;
      }
    },

    async saveCategory(category) {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore();
      if (!authStore.token) throw new Error('Not authenticated');

      try {
        let response;
        const config = { headers: { Authorization: `Bearer ${authStore.token}` } }; // FIX: Header
        if (category.id) {
          response = await axios.put(`/api/categories/${category.id}`, category, config); // FIX: URL
          const index = this.categories.findIndex(c => c.id === category.id);
          if (index !== -1) {
            this.categories[index] = response.data.data; // FIX: Data nesting
          }
        } else {
          response = await axios.post('/api/categories', category, config); // FIX: URL
          this.categories.push(response.data.data); // FIX: Data nesting
        }
        return response.data.data;
      } catch (error) {
        this.error = error;
        console.error('Error saving category:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },

    async deleteCategory(categoryId) {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore();
      if (!authStore.token) throw new Error('Not authenticated');

      try {
        await axios.delete(`/api/categories/${categoryId}`, { // FIX: URL
          headers: { Authorization: `Bearer ${authStore.token}` } // FIX: Header
        });
        this.categories = this.categories.filter(c => c.id !== categoryId);
      } catch (error) {
        this.error = error;
        console.error('Error deleting category:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
  },
});
