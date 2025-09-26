<script setup>
import { ref, onMounted, computed } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import Tag from 'primevue/tag';
import Dropdown from 'primevue/dropdown';
import InputNumber from 'primevue/inputnumber';
import FloatLabel from 'primevue/floatlabel';

const inventory = ref([]);
const stores = ref([]);
const selectedItem = ref(null);
const adjustmentDialog = ref(false);
const newQuantity = ref(0);

// Filters
const filterStore = ref();

// Dummy data mimicking backend response
const dummyData = [
    { inventory_id: 1, product_id: 1, product_name: 'Laptop', store_id: 1, store_name: 'Main Street Store', quantity: 25, last_restocked: '2025-09-15T10:00:00Z', updated_at: '2025-09-22T10:00:00Z' },
    { inventory_id: 2, product_id: 2, product_name: 'Mouse', store_id: 1, store_name: 'Main Street Store', quantity: 8, last_restocked: '2025-09-20T11:00:00Z', updated_at: '2025-09-22T11:00:00Z' },
    { inventory_id: 3, product_id: 3, product_name: 'Keyboard', store_id: 1, store_name: 'Main Street Store', quantity: 0, last_restocked: '2025-08-30T09:00:00Z', updated_at: '2025-09-15T09:00:00Z' },
    { inventory_id: 4, product_id: 1, product_name: 'Laptop', store_id: 2, store_name: 'Downtown Branch', quantity: 15, last_restocked: '2025-09-10T14:00:00Z', updated_at: '2025-09-21T14:00:00Z' },
    { inventory_id: 5, product_id: 4, product_name: 'Webcam', store_id: 2, store_name: 'Downtown Branch', quantity: 40, last_restocked: '2025-09-10T14:00:00Z', updated_at: '2025-09-18T14:00:00Z' },
    { inventory_id: 6, product_id: 3, product_name: 'Keyboard', store_id: 3, store_name: 'Uptown Plaza', quantity: 12, last_restocked: '2025-09-18T12:00:00Z', updated_at: '2025-09-22T12:00:00Z' },
];

onMounted(() => {
  inventory.value = dummyData;
  // Create a unique list of stores for the filter dropdown
  const storeSet = new Set(dummyData.map(item => item.store_name));
  stores.value = ['All Stores', ...Array.from(storeSet)];
});

const filteredInventory = computed(() => {
    if (!filterStore.value || filterStore.value === 'All Stores') {
        return inventory.value;
    }
    return inventory.value.filter(item => item.store_name === filterStore.value);
});

const openAdjustmentDialog = (item) => {
  selectedItem.value = item;
  newQuantity.value = item.quantity;
  adjustmentDialog.value = true;
};

const saveAdjustment = () => {
    if (selectedItem.value) {
        const index = inventory.value.findIndex(item => item.inventory_id === selectedItem.value.inventory_id);
        if (index > -1) {
            inventory.value[index].quantity = newQuantity.value;
            inventory.value[index].updated_at = new Date().toISOString();
        }
    }
    adjustmentDialog.value = false;
    selectedItem.value = null;
};

const formatDate = (value) => {
  if (!value) return 'N/A';
  return new Date(value).toLocaleString('en-US', { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
};

const getStockLevel = (quantity) => {
    if (quantity > 10) return { text: 'In Stock', severity: 'success' };
    if (quantity > 0) return { text: 'Low Stock', severity: 'warning' };
    return { text: 'Out of Stock', severity: 'danger' };
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Inventory Overview</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Dropdown v-model="filterStore" :options="stores" placeholder="Filter by Store" class="w-full md:w-20rem" />
          </template>
        </Toolbar>

        <DataTable :value="filteredInventory" responsiveLayout="scroll" paginator :rows="10">
          <template #empty>
            <div class="text-center py-8">
                <i class="pi pi-box text-4xl text-gray-400 mb-2"></i>
                <h3 class="text-xl font-semibold text-gray-600">No Inventory Found</h3>
                <p class="text-gray-500">No inventory items match the current filter.</p>
            </div>
          </template>

          <Column field="product_name" header="Product" :sortable="true"></Column>
          <Column field="store_name" header="Store" :sortable="true"></Column>
          <Column field="quantity" header="Quantity" :sortable="true"></Column>
          <Column header="Stock Level">
            <template #body="slotProps">
              <Tag :value="getStockLevel(slotProps.data.quantity).text" :severity="getStockLevel(slotProps.data.quantity).severity" />
            </template>
          </Column>
          <Column field="updated_at" header="Last Updated" :sortable="true">
            <template #body="slotProps">{{ formatDate(slotProps.data.updated_at) }}</template>
          </Column>
          <Column headerStyle="width: 12rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button label="Adjust" icon="pi pi-pencil" severity="secondary" @click="openAdjustmentDialog(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-if="selectedItem" v-model:visible="adjustmentDialog" :style="{width: '450px'}" header="Adjust Quantity" :modal="true">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <div class="font-semibold text-lg">{{ selectedItem.product_name }}</div>
            <div class="text-gray-500">{{ selectedItem.store_name }}</div>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber id="current_quantity" :modelValue="selectedItem.quantity" readonly fluid />
                <label for="current_quantity">Current Quantity</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber id="new_quantity" v-model="newQuantity" autofocus fluid />
                <label for="new_quantity">New Quantity</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="adjustmentDialog = false"/>
            <Button label="Save Adjustment" icon="pi pi-check" text @click="saveAdjustment" />
        </template>
    </Dialog>

  </div>
</template>
