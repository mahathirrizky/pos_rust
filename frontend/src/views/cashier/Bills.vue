<script setup>
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { useBillStore } from '../../store/bill';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Button from 'primevue/button';
import Tag from 'primevue/tag';

const billStore = useBillStore();
const { bills, isLoading, error } = storeToRefs(billStore);

onMounted(() => {
  billStore.fetchCashierOrders();
});

const formatCurrency = (value) => {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' }).format(value);
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('id-ID');
};

const getStatusSeverity = (status) => {
  switch (status.toLowerCase()) {
    case 'completed':
      return 'success';
    case 'pending':
      return 'warning';
    case 'cancelled':
      return 'danger';
    default:
      return 'info';
  }
};

const viewDetails = (bill) => {
  // Placeholder for navigation to a detailed bill view
  console.log('Viewing details for bill:', bill.id);
  // router.push({ name: 'CashierBillDetails', params: { id: bill.id } });
};

</script>

<template>
  <div class="card">
    <h1 class="text-2xl font-bold mb-6">My Recent Bills</h1>

    <div v-if="error" class="p-4 mb-4 text-sm text-red-700 bg-red-100 rounded-lg">
      <p>Error loading bills: {{ error.message }}</p>
    </div>

    <DataTable :value="bills" :loading="isLoading" responsiveLayout="scroll" paginator :rows="10">
      <template #empty> No bills found. </template>
      <template #loading> Loading bills data. Please wait. </template>

      <Column field="id" header="Order ID" sortable style="width: 10%"></Column>
      
      <Column field="order_date" header="Date" sortable style="width: 25%">
        <template #body="slotProps">
          {{ formatDate(slotProps.data.order_date) }}
        </template>
      </Column>

      <Column field="total_amount" header="Total" sortable style="width: 20%">
        <template #body="slotProps">
          {{ formatCurrency(slotProps.data.total_amount) }}
        </template>
      </Column>

      <Column field="status" header="Status" sortable style="width: 20%">
        <template #body="slotProps">
          <Tag :value="slotProps.data.status" :severity="getStatusSeverity(slotProps.data.status)" />
        </template>
      </Column>

      <Column header="Actions" style="width: 15%">
        <template #body="slotProps">
          <Button icon="pi pi-eye" label="Details" class="p-button-text p-button-sm" @click="viewDetails(slotProps.data)" />
        </template>
      </Column>

    </DataTable>
  </div>
</template>

<style scoped>
.card {
  background: var(--surface-card);
  padding: 2rem;
  border-radius: 10px;
  margin-bottom: 1rem;
}
</style>
