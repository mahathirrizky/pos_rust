<script setup>
import { ref, onMounted } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import InputNumber from 'primevue/inputnumber';
import DatePicker from 'primevue/datepicker';
import ToggleSwitch from 'primevue/toggleswitch';

import FloatLabel from 'primevue/floatlabel';
import { usePromotionStore } from '../../store/promotion';

const confirm = useConfirm();
const toast = useToast();
const promotionStore = usePromotionStore();

const selectedPromotions = ref([]);
const promotionDialog = ref(false);
const submitted = ref(false);
const promotion = ref({});
const promotionTypes = ref(['Percentage', 'FixedAmount']);

onMounted(() => {
  promotionStore.fetchPromotions();
});

const openNew = () => {
  promotion.value = { is_active: true };
  submitted.value = false;
  promotionDialog.value = true;
};

const hideDialog = () => {
  promotionDialog.value = false;
  submitted.value = false;
};

const savePromotion = async () => {
  submitted.value = true;
  if (promotion.value.name && promotion.value.promotion_type && promotion.value.value && promotion.value.start_date && promotion.value.end_date) {
    try {
      await promotionStore.savePromotion(promotion.value);
      promotionDialog.value = false;
      promotion.value = {};
      toast.add({ severity: 'success', summary: 'Success', detail: 'Promotion saved successfully', life: 3000 });
    } catch (error) {
      toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to save promotion', life: 3000 });
    }
  }
};

const editPromotion = (promo) => {
  promotion.value = { ...promo };
  promotionDialog.value = true;
};

const confirmDeletePromotion = (promo) => {
    confirm.require({
        message: 'Are you sure you want to delete this promotion?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await promotionStore.deletePromotion(promo.id);
                toast.add({ severity: 'success', summary: 'Success', detail: 'Promotion deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete promotion', life: 3000 });
            }
        }
    });
};

const deleteSelectedPromotions = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected promotions?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedPromotions.value.map(promo => promotionStore.deletePromotion(promo.id)));
                selectedPromotions.value = [];
                toast.add({ severity: 'success', summary: 'Success', detail: 'Selected promotions deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete selected promotions', life: 3000 });
            }
        }
    });
};

const formatDate = (value) => {
  return new Date(value).toLocaleDateString('en-US', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Promotion Management (Owner)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedPromotions" :disabled="!selectedPromotions || !selectedPromotions.length" />
          </template>
        </Toolbar>

        <DataTable v-model:selection="selectedPromotions" :value="promotionStore.promotions" responsiveLayout="scroll">
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="name" header="Name" :sortable="true"></Column>
          <Column field="promotion_type" header="Type" :sortable="true"></Column>
          <Column field="value" header="Value" :sortable="true">
            <template #body="slotProps">
                <span v-if="slotProps.data.promotion_type === 'Percentage'">{{ slotProps.data.value }}%</span>
                <span v-else>${{ slotProps.data.value }}</span>
            </template>
          </Column>
          <Column field="start_date" header="Start Date" :sortable="true">
            <template #body="slotProps">{{ formatDate(slotProps.data.start_date) }}</template>
          </Column>
          <Column field="end_date" header="End Date" :sortable="true">
            <template #body="slotProps">{{ formatDate(slotProps.data.end_date) }}</template>
          </Column>
          <Column field="is_active" header="Active" :sortable="true">
            <template #body="slotProps">
                <ToggleSwitch v-model="slotProps.data.is_active" readonly />
            </template>
          </Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editPromotion(slotProps.data)" />
                <Button icon="pi pi-trash" severity="warning" rounded @click="confirmDeletePromotion(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="promotionDialog" :style="{width: '450px'}" header="Promotion Details" :modal="true" class="p-fluid">
        <div class="field">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="promotion.name" required="true" autofocus :class="{'p-invalid': submitted && !promotion.name}" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !promotion.name">Name is required.</small>
        </div>
        <div class="field">
            <FloatLabel variant="on">
                <Textarea id="description" v-model="promotion.description" rows="3" cols="20" fluid />
                <label for="description">Description</label>
            </FloatLabel>
        </div>
        <div class="field">
            <FloatLabel variant="on">
                <Select id="promotion_type" v-model="promotion.promotion_type" :options="promotionTypes" required="true" :class="{'p-invalid': submitted && !promotion.promotion_type}" fluid />
                <label for="promotion_type">Promotion Type</label>
            </FloatLabel>
        </div>
        <div class="field">
            <FloatLabel variant="on">
                <InputNumber id="value" v-model="promotion.value" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" required="true" :class="{'p-invalid': submitted && !promotion.value}" fluid />
                <label for="value">Value</label>
            </FloatLabel>
        </div>
        <div class="field">
            <FloatLabel variant="on">
                <DatePicker id="start_date" v-model="promotion.start_date" required="true" :class="{'p-invalid': submitted && !promotion.start_date}" fluid></DatePicker>
                <label for="start_date">Start Date</label>
            </FloatLabel>
        </div>
        <div class="field">
            <FloatLabel variant="on">
                <DatePicker id="end_date" v-model="promotion.end_date" required="true" :class="{'p-invalid': submitted && !promotion.end_date}" fluid></DatePicker>
                <label for="end_date">End Date</label>
            </FloatLabel>
        </div>
        <div class="field">
            <FloatLabel variant="on">
                <InputNumber id="product_id" v-model="promotion.product_id" fluid />
                <label for="product_id">Product ID (Optional)</label>
            </FloatLabel>
        </div>
        <div class="field flex items-center">
            <ToggleSwitch v-model="promotion.is_active" inputId="is_active" class="mr-2" />
            <label for="is_active">Active</label>
        </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="savePromotion" />
        </template>
    </Dialog>

  </div>
</template>
