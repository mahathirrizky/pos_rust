import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth'; // Import auth store

export const useCustomerStore = defineStore('customer', {
  state: () => ({
    customers: [],
    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchCustomers() {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore(); // Get auth store
      if (!authStore.token) {
        this.error = new Error('Not authenticated');
        this.isLoading = false;
        return;
      }

      try {
        const response = await axios.get('/api/customers', {
          headers: { Authorization: `Bearer ${authStore.token}` } // Add header
        });
        this.customers = response.data.data; // Data is nested
      } catch (error) {
        this.error = error;
        console.error('Error fetching customers:', error);
      } finally {
        this.isLoading = false;
      }
    },

    async saveCustomer(customer) {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore();
      if (!authStore.token) throw new Error('Not authenticated');

      try {
        let response;
        const config = { headers: { Authorization: `Bearer ${authStore.token}` } };
        if (customer.id) {
          response = await axios.put(`/api/customers/${customer.id}`, customer, config);
          const index = this.customers.findIndex(c => c.id === customer.id);
          if (index !== -1) {
            this.customers[index] = response.data.data;
          }
        } else {
          response = await axios.post('/api/customers', customer, config);
          this.customers.push(response.data.data);
        }
        return response.data.data;
      } catch (error) {
        this.error = error;
        console.error('Error saving customer:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },

    async deleteCustomer(customerId) {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore();
      if (!authStore.token) throw new Error('Not authenticated');

      try {
        await axios.delete(`/api/customers/${customerId}`, {
          headers: { Authorization: `Bearer ${authStore.token}` }
        });
        this.customers = this.customers.filter(c => c.id !== customerId);
      } catch (error) {
        this.error = error;
        console.error('Error deleting customer:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
  },
});
