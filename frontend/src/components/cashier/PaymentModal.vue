<script setup>
import { ref, computed, watch } from 'vue';
import Dialog from 'primevue/dialog';
import Button from 'primevue/button';
import InputNumber from 'primevue/inputnumber';

const props = defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  total: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(['update:visible', 'submit']);

const paymentMethod = ref('Cashless');
const cashReceived = ref(null);

const change = computed(() => {
  if (paymentMethod.value === 'Cash' && cashReceived.value && cashReceived.value >= props.total) {
    return cashReceived.value - props.total;
  }
  return 0;
});

const isConfirmDisabled = computed(() => {
  if (paymentMethod.value === 'Cash') {
    return !cashReceived.value || cashReceived.value < props.total;
  }
  return false;
});

const closeModal = () => {
  emit('update:visible', false);
};

const handleSubmit = () => {
  emit('submit', {
    paymentMethod: paymentMethod.value,
    amountPaid: paymentMethod.value === 'Cash' ? cashReceived.value : props.total,
  });
  closeModal();
};

watch(() => props.visible, (newVal) => {
  if (newVal) {
    // Reset state when modal opens
    paymentMethod.value = 'Cashless';
    cashReceived.value = null;
  }
});

</script>

<template>
  <Dialog :visible="visible" @update:visible="closeModal" modal header="Process Payment" :style="{ width: '30rem' }">
    <div class="flex flex-col gap-6 p-4">
      <div class="text-center">
        <p class="text-lg text-surface-500">Total Amount Due</p>
        <p class="text-4xl font-bold">Rp{{ total.toFixed(2) }}</p>
      </div>

      <div class="flex justify-center gap-2">
        <Button :label="`Cashless`" :severity="paymentMethod === 'Cashless' ? 'primary' : 'secondary'" @click="paymentMethod = 'Cashless'" class="flex-1" />
        <Button :label="`Cash`" :severity="paymentMethod === 'Cash' ? 'primary' : 'secondary'" @click="paymentMethod = 'Cash'" class="flex-1" />
      </div>

      <div v-if="paymentMethod === 'Cash'" class="flex flex-col gap-4">
        <div>
          <label for="cashReceived" class="block text-sm font-medium text-surface-700 mb-2">Cash Received</label>
          <InputNumber id="cashReceived" v-model="cashReceived" mode="currency" currency="IDR" locale="id-ID" class="w-full" inputClass="text-xl" />
        </div>
        <div v-if="cashReceived && cashReceived >= total" class="text-center bg-surface-100 p-3 rounded-lg">
          <p class="text-lg text-surface-500">Change</p>
          <p class="text-3xl font-bold">Rp{{ change.toFixed(2) }}</p>
        </div>
      </div>

      <Button label="Confirm Payment" icon="pi pi-check" @click="handleSubmit" :disabled="isConfirmDisabled" class="w-full p-button-lg mt-4" />
    </div>
  </Dialog>
</template>
