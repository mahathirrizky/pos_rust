<script setup>
import { ref, nextTick } from 'vue';
import ProductList from '../../components/cashier/ProductList.vue';
import Cart from '../../components/cashier/Cart.vue';

const productListRef = ref(null);

// No local cart state or methods needed here anymore
// Cart state is managed by useCartStore

// This function is still needed to re-focus the input after adding a product
const handleProductAdded = () => {
  nextTick(() => {
    if (productListRef.value) {
      productListRef.value.focusInput();
    }
  });
};

</script>

<template>
  <div class="flex h-full gap-6">
    <div class="flex-1">
      <ProductList ref="productListRef" @product-added="handleProductAdded" />
    </div>
    <div class="w-96">
      <Cart />
    </div>
  </div>
</template>

<style scoped>
/* Add any additional styling if needed */
</style>
