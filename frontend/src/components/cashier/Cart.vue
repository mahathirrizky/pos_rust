<script setup>
import { computed } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Button from 'primevue/button';

const props = defineProps({
  cartItems: {
    type: Array,
    required: true
  }
});

const emit = defineEmits(['increase-quantity', 'decrease-quantity', 'remove-item', 'clear-cart']);

const total = computed(() => {
  return props.cartItems.reduce((acc, item) => acc + (item.price * item.quantity), 0).toFixed(2);
});

const increaseQuantity = (productId) => {
  emit('increase-quantity', productId);
};

const decreaseQuantity = (productId) => {
  emit('decrease-quantity', productId);
};

const removeItem = (productId) => {
  emit('remove-item', productId);
};

const handleClearCart = () => {
  emit('clear-cart');
};

const handlePayment = () => {
  if (props.cartItems.length === 0) {
    alert('Cart is empty!');
    return;
  }
  console.log('Processing payment for:', props.cartItems);
  console.log('Total:', total.value);
  alert(`Processing payment for a total of Rp${total.value}`);
  // Here you would typically call a backend API to create an order.
};

</script>

<template>
  <div class="card h-full flex flex-col">
    <h2 class="text-2xl font-bold mb-4 p-4">Cart</h2>
    <div class="flex-grow overflow-auto">
      <DataTable :value="cartItems" responsiveLayout="scroll">
        <Column field="name" header="Product"></Column>
        <Column header="Qty" style="width: 10rem">
          <template #body="slotProps">
            <div class="flex items-center">
              <Button icon="pi pi-minus" text rounded severity="danger" class="p-button-sm" @click="decreaseQuantity(slotProps.data.id)" />
              <span class="mx-2 font-bold">{{ slotProps.data.quantity }}</span>
              <Button icon="pi pi-plus" text rounded severity="success" class="p-button-sm" @click="increaseQuantity(slotProps.data.id)" />
            </div>
          </template>
        </Column>
        <Column field="price" header="Price">
          <template #body="slotProps">
            Rp{{ slotProps.data.price }}
          </template>
        </Column>
        <Column header="Subtotal">
          <template #body="slotProps">
            Rp{{ (slotProps.data.price * slotProps.data.quantity).toFixed(2) }}
          </template>
        </Column>
        <Column style="width: 4rem">
          <template #body="slotProps">
            <Button icon="pi pi-trash" text rounded severity="danger" @click="removeItem(slotProps.data.id)" />
          </template>
        </Column>
      </DataTable>
      <div v-if="cartItems.length === 0" class="p-4 text-center text-gray-500">
        Cart is empty.
      </div>
    </div>
    <div class="p-4 border-t">
      <div class="flex justify-between items-center mb-4">
        <span class="text-2xl font-bold">Total:</span>
        <span class="text-2xl font-bold">Rp{{ total }}</span>
      </div>
      <div class="flex gap-2">
        <Button label="Clear Cart" severity="danger" outlined class="flex-grow" @click="handleClearCart" :disabled="cartItems.length === 0" />
        <Button label="Process Payment" class="flex-grow" @click="handlePayment" :disabled="cartItems.length === 0" />
      </div>
    </div>
  </div>
</template>
