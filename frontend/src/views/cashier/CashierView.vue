<script setup>
import { ref } from 'vue';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';

import ProductList from '../../components/cashier/ProductList.vue';
import Cart from '../../components/cashier/Cart.vue';

const cart = ref([]);

const handleAddToCart = (product) => {
  const existingProduct = cart.value.find(item => item.id === product.id);
  if (existingProduct) {
    existingProduct.quantity++;
  } else {
    cart.value.push({ ...product, quantity: 1 });
  }
};

const handleIncreaseQuantity = (productId) => {
  const product = cart.value.find(item => item.id === productId);
  if (product) {
    product.quantity++;
  }
};

const handleDecreaseQuantity = (productId) => {
  const product = cart.value.find(item => item.id === productId);
  if (product && product.quantity > 1) {
    product.quantity--;
  } else if (product) {
    // If quantity is 1 or less, remove the item
    cart.value = cart.value.filter(item => item.id !== productId);
  }
};

const handleRemoveItem = (productId) => {
  cart.value = cart.value.filter(item => item.id !== productId);
};

const handleClearCart = () => {
  cart.value = [];
};

</script>

<template>
  <div class="card">
    <Splitter style="height: calc(100vh - 100px)">
      <SplitterPanel :size="65">
        <ProductList @add-to-cart="handleAddToCart" />
      </SplitterPanel>
      <SplitterPanel :size="35">
        <Cart 
          :cart-items="cart"
          @increase-quantity="handleIncreaseQuantity"
          @decrease-quantity="handleDecreaseQuantity"
          @remove-item="handleRemoveItem"
          @clear-cart="handleClearCart"
        />
      </SplitterPanel>
    </Splitter>
  </div>
</template>
