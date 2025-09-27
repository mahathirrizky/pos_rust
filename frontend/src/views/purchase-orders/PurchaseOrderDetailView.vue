<script setup>
import { ref, onMounted, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useToast } from 'primevue/usetoast';
import { useConfirm } from "primevue/useconfirm";
import { usePurchaseOrderStore } from '../../store/purchaseOrder';
import { useProductStore } from '../../store/product';
import { useAuthStore } from '../../store/auth';

import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Tag from 'primevue/tag';
import Dialog from 'primevue/dialog';
import Dropdown from 'primevue/dropdown';
import InputNumber from 'primevue/inputnumber';
import FloatLabel from 'primevue/floatlabel';

const route = useRoute();
const toast = useToast();
const confirm = useConfirm();
const purchaseOrderStore = usePurchaseOrderStore();
const productStore = useProductStore();
const authStore = useAuthStore();

const po = computed(() => purchaseOrderStore.detailedPurchaseOrder);
const itemDialog = ref(false);
const receiveDialog = ref(false);
const newItem = ref({});
const receivingItems = ref([]);
const submitted = ref(false);

const canWrite = computed(() => { 
    const writeRoles = ['Admin', 'Inventory Manager', 'Store Manager'];
    return writeRoles.includes(authStore.userRole);
});

watch(po, (newVal) => {
    if (newVal && newVal.items) {
        receivingItems.value = JSON.parse(JSON.stringify(newVal.items)).map(item => ({
            ...item,
            quantity_to_receive: item.quantity_ordered - item.quantity_received
        }));
    }
});

onMounted(() => {
  const poId = route.params.id;
  purchaseOrderStore.fetchPurchaseOrder(poId);
  productStore.fetchProducts(); // Fetch products for the dropdown
});

const openNewItemDialog = () => {
    newItem.value = {};
    submitted.value = false;
    itemDialog.value = true;
};

const hideItemDialog = () => {
    itemDialog.value = false;
    submitted.value = false;
};

const saveItem = async () => {
    submitted.value = true;
    if (!newItem.value.product_id || !newItem.value.quantity_ordered || !newItem.value.unit_price) {
        toast.add({ severity: 'error', summary: 'Validation Error', detail: 'All fields are required.', life: 3000 });
        return;
    }

    try {
        await purchaseOrderStore.addItemToPurchaseOrder(po.value.purchase_order.id, newItem.value);
        toast.add({ severity: 'success', summary: 'Success', detail: 'Item added to Purchase Order', life: 3000 });
        hideItemDialog();
    } catch (err) {
        toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to add item.', life: 3000 });
    }
};

const confirmUpdateStatus = (newStatus) => {
    confirm.require({
        message: `Are you sure you want to mark this PO as '${newStatus}'?`,
        header: 'Confirm Status Change',
        icon: 'pi pi-info-circle',
        accept: async () => {
            try {
                await purchaseOrderStore.updatePurchaseOrderStatus(po.value.purchase_order.id, newStatus);
                toast.add({ severity: 'success', summary: 'Success', detail: 'Purchase Order status updated.', life: 3000 });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to update status.', life: 3000 });
            }
        },
    });
};

const openReceiveDialog = () => {
    receiveDialog.value = true;
};

const submitReceiveStock = async () => {
    const itemsToReceive = receivingItems.value
        .filter(item => item.quantity_to_receive > 0)
        .map(item => ({
            purchase_order_item_id: item.id,
            quantity_received: item.quantity_to_receive
        }));

    if (itemsToReceive.length === 0) {
        toast.add({ severity: 'info', summary: 'No Items', detail: 'No quantities were entered to receive.', life: 3000 });
        return;
    }

    try {
        await purchaseOrderStore.receiveStock(po.value.purchase_order.id, itemsToReceive);
        toast.add({ severity: 'success', summary: 'Success', detail: 'Stock received successfully.', life: 3000 });
        receiveDialog.value = false;
    } catch (err) {
        toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to receive stock.', life: 3000 });
    }
};

const getStatusSeverity = (status) => {
    if (!status) return 'info';
    const statuses = {
        draft: 'info',
        ordered: 'warning',
        partially_received: 'primary',
        completed: 'success',
        cancelled: 'danger'
    };
    return statuses[status.toLowerCase()] || 'info';
};

const formatCurrency = (value) => {
    if (typeof value !== 'number') {
        value = parseFloat(value);
    }
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

const formatDate = (value) => {
    if (!value) return 'N/A';
    const date = new Date(value);
    return date.toLocaleDateString('en-US', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
    });
};

</script>

<template>
  <div v-if="!po">
    <p>Loading purchase order...</p>
  </div>
  <Card v-else>
    <template #title>
      <div class="flex justify-between items-center">
        <span class="text-2xl font-semibold">Purchase Order #{{ po.purchase_order.id }}</span>
        <Tag :value="po.purchase_order.status" :severity="getStatusSeverity(po.purchase_order.status)" />
      </div>
    </template>
    <template #subtitle>
        <div class="flex items-center gap-2 mt-2">
            <Button v-if="canWrite && po.purchase_order.status === 'draft'" label="Mark as Ordered" icon="pi pi-check-circle" @click="confirmUpdateStatus('ordered')" severity="success"/>
            <Button v-if="canWrite && ['ordered', 'partially_received'].includes(po.purchase_order.status)" label="Receive Stock" icon="pi pi-truck" @click="openReceiveDialog" severity="info"/>
        </div>
    </template>
    <template #content>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <div>
          <h3 class="font-semibold text-lg mb-2">Supplier</h3>
          <p>{{ po.supplier?.name || 'N/A' }}</p>
        </div>
        <div>
          <h3 class="font-semibold text-lg mb-2">Store</h3>
          <p>{{ po.store?.name || 'N/A' }}</p>
        </div>
        <div>
          <h3 class="font-semibold text-lg mb-2">Dates</h3>
          <p><strong>Order Date:</strong> {{ formatDate(po.purchase_order.order_date) }}</p>
          <p><strong>Expected Delivery:</strong> {{ formatDate(po.purchase_order.expected_delivery_date) }}</p>
        </div>
      </div>

      <div class="flex justify-between items-center mb-4">
        <h3 class="text-xl font-semibold">Items</h3>
        <Button v-if="canWrite && po.purchase_order.status === 'draft'" label="Add Item" icon="pi pi-plus" @click="openNewItemDialog" />
      </div>
      <DataTable :value="po.items" responsiveLayout="scroll">
        <Column field="product.name" header="Product"></Column>
        <Column field="quantity_ordered" header="Ordered"></Column>
        <Column field="quantity_received" header="Received"></Column>
        <Column field="unit_price" header="Unit Price">
          <template #body="slotProps">
            {{ formatCurrency(slotProps.data.unit_price) }}
          </template>
        </Column>
        <Column header="Total">
          <template #body="slotProps">
            {{ formatCurrency(slotProps.data.quantity_ordered * parseFloat(slotProps.data.unit_price)) }}
          </template>
        </Column>
      </DataTable>

    </template>
  </Card>

  <Dialog v-model:visible="itemDialog" :style="{width: '450px'}" header="Add Item to PO" :modal="true" class="p-fluid">
    <div class="field mt-4">
        <FloatLabel>
            <Select id="product" v-model="newItem.product_id" :options="productStore.products" optionLabel="name" optionValue="id" placeholder="Select a Product" :class="{'p-invalid': submitted && !newItem.product_id}" filter></Select>
            <label for="product">Product</label>
        </FloatLabel>
    </div>
    <div class="field mt-4">
        <FloatLabel>
            <InputNumber id="quantity" v-model="newItem.quantity_ordered" mode="decimal" :class="{'p-invalid': submitted && !newItem.quantity_ordered}" />
            <label for="quantity">Quantity Ordered</label>
        </FloatLabel>
    </div>
    <div class="field mt-4">
        <FloatLabel>
            <InputNumber id="price" v-model="newItem.unit_price" mode="currency" currency="USD" locale="en-US" :class="{'p-invalid': submitted && !newItem.unit_price}"/>
            <label for="price">Unit Price</label>
        </FloatLabel>
    </div>
    <template #footer>
        <Button label="Cancel" icon="pi pi-times" text @click="hideItemDialog"/>
        <Button label="Save" icon="pi pi-check" text @click="saveItem" />
    </template>
  </Dialog>

  <Dialog v-model:visible="receiveDialog" :style="{width: '750px'}" header="Receive Stock" :modal="true">
    <DataTable :value="receivingItems">
        <Column field="product.name" header="Product"></Column>
        <Column field="quantity_ordered" header="Ordered"></Column>
        <Column field="quantity_received" header="Already Received"></Column>
        <Column header="Receiving Now">
            <template #body="{ data }">
                <InputNumber v-model="data.quantity_to_receive" mode="decimal" showButtons :min="0" :max="data.quantity_ordered - data.quantity_received" />
            </template>
        </Column>
    </DataTable>
    <template #footer>
        <Button label="Cancel" icon="pi pi-times" text @click="receiveDialog = false"/>
        <Button label="Submit Received Stock" icon="pi pi-check" text @click="submitReceiveStock" />
    </template>
  </Dialog>
</template>
