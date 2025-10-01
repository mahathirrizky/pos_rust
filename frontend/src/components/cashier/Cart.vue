<script setup>
import { computed, ref } from 'vue';
import Button from 'primevue/button';
import InputNumber from 'primevue/inputnumber';
import { useToast } from 'primevue/usetoast';
import { useBillStore } from '../../store/bill';
import { useCartStore } from '../../store/cart';
import { storeToRefs } from 'pinia';
import PaymentModal from './PaymentModal.vue';

const toast = useToast();
const billStore = useBillStore();
const cartStore = useCartStore();
const { cart } = storeToRefs(cartStore);

const showPaymentModal = ref(false);

const total = computed(() => {
  return cart.value.reduce((acc, item) => acc + (item.price * item.quantity), 0);
});

const handleQuantityChange = (item, newQuantity) => {
  cartStore.updateQuantity(item.id, newQuantity);
};

const removeItem = (productId) => {
  cartStore.remove(productId);
};

const handleClearCart = () => {
  cartStore.clear();
};

const handleProcessPayment = () => {
  if (cart.value.length === 0) {
    toast.add({ severity: 'warn', summary: 'Empty Cart', detail: 'Cannot process payment for an empty cart.', life: 3000 });
    return;
  }
  showPaymentModal.value = true;
};

const handleConfirmPayment = async (paymentDetails) => {
  const payload = {
    customer_id: 1, // Assuming a default customer for now
    items: cart.value.map(item => ({
      product_id: item.id,
      quantity: item.quantity,
      promotion_id: null, // Ignoring promotions for now
    })),
    payment_method: paymentDetails.paymentMethod,
  };

  try {
    await billStore.submitOrder(payload);
    toast.add({ severity: 'success', summary: 'Payment Successful', detail: 'Order has been created.', life: 3000 });
    cartStore.clear(); // Clear cart after successful payment
  } catch (err) {
    const errorMessage = err.response?.data?.message || err.message || 'Failed to create order.';
    toast.add({ severity: 'error', summary: 'Payment Failed', detail: errorMessage, life: 3000 });
  }
};

</script>

<template>
  <div class="bg-surface-0 dark:bg-surface-800 rounded-lg shadow-md h-full flex flex-col">
    <div class="p-4 border-b border-surface-200">
      <h2 class="text-xl font-bold">Current Order</h2>
    </div>

    <!-- Customer Info Placeholder -->
    <div class="p-4 border-b border-surface-200">
      <div class="flex items-center">
        <i class="pi pi-user text-2xl text-surface-500 mr-3"></i>
        <div>
          <p class="font-semibold">Guest Customer</p>
          <a href="#" class="text-sm text-primary-500 hover:underline">Add / Change Customer</a>
        </div>
      </div>
    </div>

    <!-- Cart Items -->
    <div class="flex-grow overflow-y-auto p-4">
      <div v-if="cart.length === 0" class="text-center text-surface-500 pt-16">
        <i class="pi pi-shopping-cart text-4xl"></i>
        <p class="mt-2">Your cart is empty</p>
      </div>
      <div v-else class="space-y-3">
        <div v-for="item in cart" :key="item.id" class="flex items-center">
          <div class="flex-grow">
            <p class="font-semibold text-sm">{{ item.name }}</p>
            <p class="text-xs text-surface-500">Rp{{ item.price }}</p>
          </div>
          <div class="w-32">
            <InputNumber 
              :modelValue="item.quantity" 
              @update:modelValue="(val) => handleQuantityChange(item, val)" 
              showButtons 
              buttonLayout="horizontal" 
              :min="0" 
              :step="1" 
              inputClass="w-12 text-center" 
              decrementButtonClass="p-button-secondary" 
              incrementButtonClass="p-button-secondary" 
              incrementButtonIcon="pi pi-plus" 
              decrementButtonIcon="pi pi-minus" 
            />
          </div>
          <p class="font-semibold text-sm w-24 text-right">Rp{{ (item.price * item.quantity).toFixed(2) }}</p>
          <Button icon="pi pi-trash" text rounded severity="danger" class="p-button-sm ml-2" @click="removeItem(item.id)" />
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-surface-200 mt-auto">
      <div class="flex justify-between items-center mb-4">
        <span class="text-lg font-bold">Total:</span>
        <span class="text-2xl font-bold">Rp{{ total.toFixed(2) }}</span>
      </div>
      <Button 
        label="Process Payment" 
        icon="pi pi-check" 
        class="w-full p-button-lg" 
        @click="handleProcessPayment" 
        :disabled="cart.length === 0" 
      />
      <Button 
        label="Clear Cart" 
        severity="secondary" 
        text 
        class="w-full mt-2" 
        @click="handleClearCart" 
        :disabled="cart.length === 0" 
      />
    </div>

    <PaymentModal v-model:visible="showPaymentModal" :total="total" @submit="handleConfirmPayment" />
  </div>
</template>