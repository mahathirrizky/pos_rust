<script setup>
import { ref, onMounted, computed } from 'vue';
import Button from 'primevue/button';
import ProgressSpinner from 'primevue/progressspinner';
import InputText from 'primevue/inputtext';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Card from 'primevue/card';
import { useProductStore } from '../../store/product';
import { useInventoryStore } from '../../store/inventory'; // Still need this to trigger fetch

const productStore = useProductStore();
const inventoryStore = useInventoryStore(); // Keep this to call fetchInventory

const searchQuery = ref('');
const searchInput = ref(null);

const emit = defineEmits(['add-to-cart']);

// The search filter now uses the new getter from the product store
const filteredProducts = computed(() => {
  // The getter is already reactive and contains the merged data
  const source = productStore.productsWithRealtimeStock;
  if (!searchQuery.value) {
    return source;
  }
  return source.filter(product =>
    product.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    (product.sku && product.sku.toLowerCase().includes(searchQuery.value.toLowerCase()))
  );
});

onMounted(() => {
  // Fetch from both stores to populate them initially.
  // The inventory store will then keep itself updated via WebSocket.
  productStore.fetchProducts();
  inventoryStore.fetchInventory();

  if (searchInput.value && searchInput.value.$el) {
    searchInput.value.$el.focus();
  }
});

const handleAddToCart = (product) => {
  emit('add-to-cart', product);
};
</script>

<template>
  <div class="card h-full flex flex-col">
    <div class="p-4">
      <IconField iconPosition="left" class="w-full">
        <InputIcon class="pi pi-search" />
        <InputText ref="searchInput" v-model="searchQuery" placeholder="Search products by name or SKU" fluid />
      </IconField>
    </div>

    <div class="flex-grow overflow-auto">
      <div v-if="!filteredProducts.length && !searchQuery" class="flex justify-center items-center h-full">
        <ProgressSpinner />
      </div>
      <div v-else-if="filteredProducts.length > 0">
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 p-2">
          <div v-for="product in filteredProducts" :key="product.id" class="p-2">
            <Card class="h-full flex flex-col">
              <template #header>
                <div class="bg-surface-100 flex items-center justify-center h-40 rounded-t-lg">
                  <i class="pi pi-image text-6xl text-surface-400"></i>
                </div>
              </template>
              <template #content>
                <div class="flex flex-col flex-grow">
                  <h5 class="text-lg font-bold">{{ product.name }}</h5>
                  <p class="text-md line-clamp-1">{{ product.description }}</p>
                  <p class="text-sm text-surface-500 mt-1">SKU: {{ product.sku }}</p>
                  <div class="flex flex-col mt-auto">
                    <span class="text-2xl font-semibold text-primary">Rp{{ product.price }}</span>
                    <span v-if="product.stock > 0" :class="{'text-red-500': product.stock < 10, 'text-green-500': product.stock >= 10}" class="text-sm mt-1">
                      Stock: {{ product.stock }}
                    </span>
                    <span v-else class="text-red-600 font-bold text-sm mt-1">
                      Out of Stock
                    </span>
                  </div>
                </div>
              </template>
              <template #footer>
                <Button label="Add" icon="pi pi-plus" class="mt-4 w-full" @click="handleAddToCart(product)" :disabled="product.stock <= 0" />
              </template>
            </Card>
          </div>
        </div>
      </div>
      <div v-else class="p-4 text-center text-gray-500">
        No products found.
      </div>
    </div>
  </div>
</template>