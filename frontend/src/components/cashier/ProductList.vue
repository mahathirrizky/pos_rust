<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';

import InputText from 'primevue/inputtext';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import DataView from 'primevue/dataview';

import { useToast } from 'primevue/usetoast';
import { useProductStore } from '../../store/product';
import { useInventoryStore } from '../../store/inventory';
import { useCartStore } from '../../store/cart';

const productStore = useProductStore();
const inventoryStore = useInventoryStore();
const cartStore = useCartStore();
const toast = useToast();

const searchQuery = ref('');
const searchInput = ref(null);
const layout = ref('grid');

const emit = defineEmits(['product-added']);

const filteredProducts = computed(() => {
  const source = productStore.productsWithRealtimeStock;
  if (!searchQuery.value) {
    return source;
  }
  return source.filter(product =>
    product.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    (product.sku && product.sku.toLowerCase().includes(searchQuery.value.toLowerCase()))
  );
});

const focusInput = () => {
  if (searchInput.value && searchInput.value.$el) {
    searchInput.value.$el.focus();
  }
};

defineExpose({
  focusInput
});

const handleGlobalKeyDown = (event) => {
  const target = event.target;
  // Ignore if the event is already coming from an input, textarea, or a button
  if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.tagName === 'BUTTON') {
    return;
  }

  // Check if the key is a printable character (and not a control key like Shift, etc.)
  if (event.key.length === 1) {
    focusInput();
  }
};

onMounted(() => {
  productStore.fetchProducts();
  inventoryStore.fetchInventory();
  focusInput(); // Initial focus
  window.addEventListener('keydown', handleGlobalKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeyDown);
});

const handleAddToCart = (product) => {
  if (product.stock > 0) {
    cartStore.add(product);
    emit('product-added');
  }
};

const handleBarcodeScan = () => {
  const query = searchQuery.value.trim();
  if (!query) return;

  const allProducts = productStore.productsWithRealtimeStock;
  const matchedProduct = allProducts.find(p => p.sku === query);

  if (matchedProduct) {
    if (matchedProduct.stock > 0) {
      cartStore.add(matchedProduct);
      searchQuery.value = ''; // Clear for next scan
      emit('product-added');
    } else {
      toast.add({ severity: 'warn', summary: 'Out of Stock', detail: `Product "${matchedProduct.name}" is out of stock.`, life: 3000 });
    }
  } else {
    toast.add({ severity: 'error', summary: 'Not Found', detail: `No product found with SKU "${query}".`, life: 3000 });
  }
};

</script>

<template>
  <div class="bg-surface-0 dark:bg-surface-800 rounded-lg shadow-md h-full flex flex-col">
    <div class="p-4 border-b border-surface-200">
      <h2 class="text-xl font-bold">Products</h2>
      <IconField iconPosition="left" class="w-full mt-4">
        <InputIcon class="pi pi-search" />
        <InputText 
          ref="searchInput" 
          v-model="searchQuery" 
          placeholder="Search or scan barcode..." 
          class="w-full" 
          @keydown.enter="handleBarcodeScan"
        />
      </IconField>
    </div>

    <DataView :value="filteredProducts" :layout="layout" paginator :rows="12" class="flex-grow overflow-hidden">
        <template #header>
            <div class="flex justify-end">
               
            </div>
        </template>

        <template #list="slotProps">
            <div class="grid grid-cols-1 gap-2">
                <div v-for="(item, index) in slotProps.items" :key="index" 
                    @click="handleAddToCart(item)"
                    :class="['flex p-3 items-center gap-4 transition-shadow duration-200 rounded-lg border', {
                        'cursor-pointer hover:shadow-md': item.stock > 0,
                        'opacity-50 cursor-not-allowed': item.stock <= 0
                    }]"
                >
                    <div class="flex-1 flex flex-col gap-1">
                        <div class="font-bold">{{ item.name }}</div>
                        <div class="text-sm text-surface-500">SKU: {{ item.sku }}</div>
                    </div>
                    <div :class="['text-sm', {
                        'text-red-500': item.stock < 10 && item.stock > 0,
                        'text-green-500': item.stock >= 10,
                        'text-red-600 font-bold': item.stock <= 0
                    }]">
                        {{ item.stock > 0 ? `${item.stock} in stock` : 'Out of Stock' }}
                    </div>
                    <div class="text-lg font-semibold">Rp{{ item.price }}</div>
                </div>
            </div>
        </template>

        <template #grid="slotProps">
            <div class="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4">
                <div v-for="(item, index) in slotProps.items" :key="index"
                    @click="handleAddToCart(item)"
                    :class="['border rounded-lg p-3 flex flex-col text-center transition-shadow duration-200', {
                        'cursor-pointer hover:shadow-lg hover:border-primary-500': item.stock > 0,
                        'opacity-50 cursor-not-allowed': item.stock <= 0
                    }]"
                >
                    <div class="font-semibold truncate">{{ item.name }}</div>
                    <div class="text-xs text-surface-500">SKU: {{ item.sku }}</div>
                    <div class="mt-4">
                        <div class="font-bold text-lg">Rp{{ item.price }}</div>
                        <div :class="['text-xs mt-1', {
                            'text-red-500': item.stock < 10 && item.stock > 0,
                            'text-green-500': item.stock >= 10,
                            'text-red-600 font-bold': item.stock <= 0
                        }]">
                            {{ item.stock > 0 ? `${item.stock} in stock` : 'Out of Stock' }}
                        </div>
                    </div>
                </div>
            </div>
        </template>

        <template #empty>
            <div class="p-4 text-center text-surface-500 dark:text-surface-400">
                <span v-if="!searchQuery">Belum ada produk.</span>
                <span v-else>No products found matching your search.</span>
            </div>
        </template>
    </DataView>
  </div>
</template>
