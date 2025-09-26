import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useSupplierStore = defineStore('supplier', {
  state: () => ({
    suppliers: [],
  }),
  actions: {
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
        this.suppliers = response.data.data;
      } catch (error) {
        console.error('Error fetching suppliers:', error);
      }
    },
    async saveSupplier(supplier) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        const isUpdate = !!supplier.id;
        const url = isUpdate ? `/api/suppliers/${supplier.id}` : '/api/suppliers';
        const method = isUpdate ? 'put' : 'post';

        try {
            await axios({
                method,
                url,
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
                data: supplier,
            });
            await this.fetchSuppliers();
        } catch (error) {
            console.error('Error saving supplier:', error);
            throw error;
        }
    },
    async deleteSupplier(supplierId) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        try {
            await axios.delete(`/api/suppliers/${supplierId}`, {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            await this.fetchSuppliers();
        } catch (error) {
            console.error('Error deleting supplier:', error);
            throw error;
        }
    },
  },
});