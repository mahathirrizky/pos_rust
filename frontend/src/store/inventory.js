import { defineStore } from 'pinia';
import axios from 'axios';
import { ref, watch } from 'vue';
import { useAuthStore } from './auth';
import { useWebSocket } from '../composables/useWebSocket';

export const useInventoryStore = defineStore('inventory', () => {
  // STATE
  const inventory = ref([]);
  const stores = ref([]);
  const inventoryReport = ref(null);
  const loading = ref(false);

  // WEBSOCKET
  // Assuming the backend runs on port 8000
  const wsUrl = `ws://${window.location.hostname}:8000/ws`;
  const { data: wsData } = useWebSocket(wsUrl);

  // ACTIONS
  const authStore = useAuthStore();

  async function fetchInventory() {
    if (!authStore.token) return;
    loading.value = true;
    try {
      const response = await axios.get('/api/inventory', {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      inventory.value = response.data.data;
    } catch (error) {
      console.error('Error fetching inventory:', error);
    } finally {
      loading.value = false;
    }
  }

  async function fetchStores() {
    if (!authStore.token) return;
    try {
      const response = await axios.get('/api/stores', {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      stores.value = ['All Stores', ...response.data.data.map(s => s.name)];
    } catch (error) {
      console.error('Error fetching stores:', error);
    }
  }

  async function adjustInventory(inventoryId, quantity) {
    if (!authStore.token) throw new Error('No token found');
    const url = `/api/inventory/${inventoryId}`;
    try {
      await axios.put(url, { quantity }, {
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${authStore.token}`,
        },
      });
      // No need to refetch here, WebSocket will trigger it.
    } catch (error) {
      console.error('Error adjusting inventory item:', error);
      throw error;
    }
  }

  async function fetchInventoryReport() {
    if (!authStore.token) return;
    try {
      const response = await axios.get('/api/inventory/report', {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      inventoryReport.value = response.data.data;
    } catch (error) {
      console.error('Error fetching inventory report:', error);
    }
  }

  async function updateInventoryItem(item) {
    if (!authStore.token) throw new Error('No token found');
    const url = `/api/inventory/${item.inventory_id}`;
    try {
      await axios.put(url, item, {
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${authStore.token}`,
        },
      });
      // No need to refetch here, WebSocket will trigger it.
    } catch (error) {
      console.error('Error updating inventory item:', error);
      throw error;
    }
  }

  // WATCHER for WebSocket messages
  watch(wsData, (newMessage) => {
    if (newMessage === 'inventory_updated') {
      console.log('Inventory update received via WebSocket. Refetching data...');
      fetchInventory();
      fetchInventoryReport();
    }
  });

  return {
    inventory,
    stores,
    inventoryReport,
    loading,
    fetchInventory,
    fetchStores,
    adjustInventory,
    fetchInventoryReport,
    updateInventoryItem,
  };
});
""