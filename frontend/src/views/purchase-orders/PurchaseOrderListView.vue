<script setup>
import { onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { usePurchaseOrderStore } from '../../store/purchaseOrder';
import { useAuthStore } from '../../store/auth';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';

const router = useRouter();
const purchaseOrderStore = usePurchaseOrderStore();
const authStore = useAuthStore();

const purchaseOrders = computed(() => purchaseOrderStore.purchaseOrders);
const loading = computed(() => purchaseOrderStore.loading);

const canWrite = computed(() => { 
    const writeRoles = ['Admin', 'Inventory Manager', 'Store Manager'];
    return writeRoles.includes(authStore.userRole);
});

onMounted(() => {
  purchaseOrderStore.fetchPurchaseOrders();
});

const openNew = () => {
  const currentPath = router.currentRoute.value.path;
  router.push(`${currentPath}/new`);
};

const viewPurchaseOrder = (po) => {
  const currentPath = router.currentRoute.value.path;
  router.push(`${currentPath}/${po.purchase_order.id}`);
};

const formatDate = (value) => {
    const date = new Date(value);
    return date.toLocaleDateString('en-US', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
    });
};

const formatCurrency = (value) => {
    if (typeof value !== 'number') {
        value = parseFloat(value);
    }
    return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' }).format(value);
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Purchase Orders</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button v-if="canWrite" label="New" icon="pi pi-plus" class="mr-2" severity="success" @click="openNew" />
          </template>
        </Toolbar>

        <DataTable :value="purchaseOrders" responsiveLayout="scroll" :loading="loading">
          <Column field="purchase_order.id" header="PO Number" :sortable="true"></Column>
          <Column field="supplier.name" header="Supplier" :sortable="true"></Column>
          <Column field="store.name" header="Store" :sortable="true"></Column>
          <Column field="purchase_order.status" header="Status" :sortable="true"></Column>
          <Column field="purchase_order.order_date" header="Order Date" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.purchase_order.order_date) }}
            </template>
          </Column>
          <Column field="purchase_order.total_amount" header="Total" :sortable="true">
             <template #body="slotProps">
              {{ formatCurrency(slotProps.data.purchase_order.total_amount) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-eye" class="p-button-rounded p-button-info" @click="viewPurchaseOrder(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>
  </div>
</template>
