
import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useProductStore = defineStore('product', {
  state: () => ({
    products: [],
    categories: [],
    suppliers: [],
  }),
  actions: {
    async fetchProducts() {
      const authStore = useAuthStore();
      if (!authStore.token) {
        console.error('No token found, user is not authenticated');
        return;
      }
      try {
        const response = await axios.get('/api/products', {
          headers: {
            Authorization: `Bearer ${authStore.token}`,
          },
        });
        this.products = response.data.data;
      } catch (error) {
        console.error('Error fetching products:', error);
      }
    },
    async fetchCategories() {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        try {
            const response = await axios.get('/api/categories', {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            this.categories = response.data.data.map(c => ({ label: c.name, value: c.id }));
        } catch (error) {
            console.error('Error fetching categories:', error);
        }
    },
    async fetchSuppliers() {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        try {
            const response = await axios.get('/api/suppliers', {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            this.suppliers = response.data.data.map(s => ({ label: s.name, value: s.id }));
        } catch (error) {
            console.error('Error fetching suppliers:', error);
        }
    },
    async saveProduct(product) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        const isUpdate = !!product.id;
        const url = isUpdate ? `/api/products/${product.id}` : '/api/products';
        const method = isUpdate ? 'put' : 'post';

        try {
            await axios({
                method,
                url,
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
                data: product,
            });
            await this.fetchProducts();
        } catch (error) {
            console.error('Error saving product:', error);
            throw error;
        }
    },
    async deleteProduct(productId) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        try {
            await axios.delete(`/api/products/${productId}`, {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            await this.fetchProducts();
        } catch (error) {
            console.error('Error deleting product:', error);
            throw error;
        }
    },
  },
});
