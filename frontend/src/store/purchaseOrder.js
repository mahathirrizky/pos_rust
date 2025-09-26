import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

// Set the base URL for all axios requests
axios.defaults.baseURL = 'http://localhost:8000';

export const usePurchaseOrderStore = defineStore('purchaseOrder', {
  state: () => ({
    purchaseOrders: [],
    detailedPurchaseOrder: null,
    loading: false,
  }),
  actions: {
    async fetchPurchaseOrders() {
      const authStore = useAuthStore();
      if (!authStore.token) return;
      this.loading = true;
      try {
        const response = await axios.get('/api/purchase-orders', {
          headers: { Authorization: `Bearer ${authStore.token}` },
        });
        this.purchaseOrders = response.data.data;
      } catch (error) {
        console.error('Error fetching purchase orders:', error);
        this.purchaseOrders = []; // Clear on error
      } finally {
        this.loading = false;
      }
    },

    async fetchPurchaseOrder(id) {
      const authStore = useAuthStore();
      if (!authStore.token) return;
      this.loading = true;
      try {
        const response = await axios.get(`/api/purchase-orders/${id}`,
        {
          headers: { Authorization: `Bearer ${authStore.token}` },
        });
        this.detailedPurchaseOrder = response.data.data;
      } catch (error) {
        console.error(`Error fetching purchase order ${id}:`, error);
        this.detailedPurchaseOrder = null;
      } finally {
        this.loading = false;
      }
    },

    async createPurchaseOrder(poData) {
        const authStore = useAuthStore();
        if (!authStore.token) throw new Error('Not authenticated');

        try {
            const response = await axios.post('/api/purchase-orders', poData, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            // After creating, refresh the list
            await this.fetchPurchaseOrders();
            return response.data.data; // Return the newly created PO
        } catch (error) {
            console.error('Error creating purchase order:', error);
            throw error; // Re-throw to be handled in the component
        }
    },

    async addItemToPurchaseOrder(poId, itemData) {
        const authStore = useAuthStore();
        if (!authStore.token) throw new Error('Not authenticated');

        try {
            await axios.post(`/api/purchase-orders/${poId}/items`, itemData, {
                 headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            // After adding an item, refresh the detailed view of the current PO
            await this.fetchPurchaseOrder(poId);
        } catch (error) {
            console.error('Error adding item to purchase order:', error);
            throw error; // Re-throw to be handled in the component
        }
    },

    async updatePurchaseOrderStatus(poId, newStatus) {
        const authStore = useAuthStore();
        if (!authStore.token) throw new Error('Not authenticated');

        try {
            await axios.put(`/api/purchase-orders/${poId}/status`, { status: newStatus }, {
                 headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            // After updating, refresh the detailed view of the current PO
            await this.fetchPurchaseOrder(poId);
        } catch (error) {
            console.error('Error updating purchase order status:', error);
            throw error; // Re-throw to be handled in the component
        }
    },

    async receiveStock(poId, items) {
        const authStore = useAuthStore();
        if (!authStore.token) throw new Error('Not authenticated');

        try {
            await axios.post(`/api/purchase-orders/${poId}/receive`, { items }, {
                 headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            // After receiving, refresh the detailed view of the current PO
            await this.fetchPurchaseOrder(poId);
        } catch (error) {
            console.error('Error receiving stock:', error);
            throw error; // Re-throw to be handled in the component
        }
    },
  },
});
