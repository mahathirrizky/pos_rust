import { ref, watch } from 'vue';
import { defineStore } from 'pinia';

export const useCartStore = defineStore('cart', () => {
  // Load cart from localStorage
  const cart = ref(JSON.parse(localStorage.getItem('cart')) || []);

  // Watch for changes and save to localStorage
  watch(cart, (newCart) => {
    localStorage.setItem('cart', JSON.stringify(newCart));
  }, { deep: true });

  // --- ACTIONS ---

  function add(product) {
    const existingProduct = cart.value.find(item => item.id === product.id);
    if (existingProduct) {
      existingProduct.quantity++;
    } else {
      cart.value.push({ ...product, quantity: 1 });
    }
  }

  function updateQuantity(productId, newQuantity) {
    const product = cart.value.find(item => item.id === productId);
    if (product) {
      if (newQuantity >= 1) {
        product.quantity = newQuantity;
      } else {
        // Remove if quantity is less than 1
        remove(productId);
      }
    }
  }

  function remove(productId) {
    cart.value = cart.value.filter(item => item.id !== productId);
  }

  function clear() {
    cart.value = [];
  }

  return {
    cart,
    add,
    updateQuantity,
    remove,
    clear,
  };
});
