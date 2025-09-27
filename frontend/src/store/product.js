
import { defineStore } from 'pinia';
import axios from 'axios';
import { ref, computed } from 'vue';
import { useAuthStore } from './auth';
import { useInventoryStore } from './inventory';

export const useProductStore = defineStore('product', () => {
  // STATE
  const products = ref([]);
  const categories = ref([]);
  const suppliers = ref([]);

  // OTHER STORES
  const authStore = useAuthStore();
  const inventoryStore = useInventoryStore();

  // ACTIONS
  async function fetchProducts() {
    if (!authStore.token) return;
    try {
      const response = await axios.get('/api/products', {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      products.value = response.data.data;
    } catch (error) {
      console.error('Error fetching products:', error);
    }
  }

  async function fetchCategories() {
    if (!authStore.token) return;
    try {
      const response = await axios.get('/api/categories', {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      categories.value = response.data.data.map(c => ({ label: c.name, value: c.id }));
    } catch (error) {
      console.error('Error fetching categories:', error);
    }
  }

  async function fetchSuppliers() {
    if (!authStore.token) return;
    try {
      const response = await axios.get('/api/suppliers', {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      suppliers.value = response.data.data.map(s => ({ label: s.name, value: s.id }));
    } catch (error) {
      console.error('Error fetching suppliers:', error);
    }
  }

  async function saveProduct(product) {
    if (!authStore.token) throw new Error('Not authenticated');
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
      await fetchProducts();
    } catch (error) {
      console.error('Error saving product:', error);
      throw error;
    }
  }

  async function deleteProduct(productId) {
    if (!authStore.token) throw new Error('Not authenticated');
    try {
      await axios.delete(`/api/products/${productId}`, {
        headers: { Authorization: `Bearer ${authStore.token}` },
      });
      await fetchProducts();
    } catch (error) {
      console.error('Error deleting product:', error);
      throw error;
    }
  }

  // GETTERS (as computed properties)
  const productsWithRealtimeStock = computed(() => {
    if (!products.value.length) return [];

    const currentStoreId = authStore.storeId;
    if (!currentStoreId) {
      console.warn("Store ID not found, showing all items as out of stock.");
      return products.value.map(p => ({ ...p, stock: 0 }));
    }

    const inventoryMap = new Map();
    if (inventoryStore.inventory && inventoryStore.inventory.length) {
      inventoryStore.inventory
        .filter(item => item.store_id === currentStoreId)
        .forEach(item => {
          inventoryMap.set(item.product_id, item.quantity);
        });
    }

    return products.value.map(product => ({
      ...product,
      stock: inventoryMap.get(product.id) || 0,
    }));
  });

  return {
    products,
    categories,
    suppliers,
    fetchProducts,
    fetchCategories,
    fetchSuppliers,
    saveProduct,
    deleteProduct,
    productsWithRealtimeStock, // Expose the new getter
  };
});
