<script setup>
import { computed } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Button from 'primevue/button';
import { useToast } from 'primevue/usetoast';
import { useAuthStore } from '../../store/auth';


const props = defineProps({
  cartItems: {
    type: Array,
    required: true
  }
});

const emit = defineEmits(['increase-quantity', 'decrease-quantity', 'remove-item', 'clear-cart']);
const toast = useToast();
const authStore = useAuthStore();

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

const handlePayment = async () => {
  if (props.cartItems.length === 0) {
    toast.add({ severity: 'warn', summary: 'Empty Cart', detail: 'Cannot process payment for an empty cart.', life: 3000 });
    return;
  }

  const payload = {
    customer_id: 1, // Assuming a default customer for now
    items: props.cartItems.map(item => ({
      product_id: item.id,
      quantity: item.quantity,
      promotion_id: null, // Ignoring promotions for now
    }))
  };

  try {
    const response = await fetch('/api/orders', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${authStore.token}`,
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      const errorData = await response.json();
      throw new Error(errorData.message || 'Failed to create order.');
    }

    toast.add({ severity: 'success', summary: 'Payment Successful', detail: 'Order has been created.', life: 3000 });
    emit('clear-cart');

  } catch (err) {
    toast.add({ severity: 'error', summary: 'Payment Failed', detail: err.message, life: 3000 });
  }
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