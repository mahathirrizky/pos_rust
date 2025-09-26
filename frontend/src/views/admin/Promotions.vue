<script setup>
import { ref, onMounted } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import InputNumber from 'primevue/inputnumber';
import Calendar from 'primevue/calendar';
import ToggleSwitch from 'primevue/toggleswitch';
import Dropdown from 'primevue/dropdown';
import FloatLabel from 'primevue/floatlabel';

const confirm = useConfirm();
const promotions = ref([]);
const selectedPromotions = ref([]);
const promotionDialog = ref(false);
const submitted = ref(false);
const promotion = ref({});
const promotionTypes = ref(['Percentage', 'FixedAmount']);

// Dummy data mimicking backend response
const dummyData = [
    {
        id: 1,
        name: '10% Off Laptops',
        description: 'Get 10% off on all laptop models.',
        promotion_type: 'Percentage',
        value: '10.00',
        start_date: '2025-10-01T00:00:00Z',
        end_date: '2025-10-31T23:59:59Z',
        is_active: true,
        product_id: 1, // Assuming this links to a product
        created_at: '2025-09-15T10:00:00Z',
        updated_at: '2025-09-15T10:00:00Z',
    },
    {
        id: 2,
        name: '$5 Off Keyboards',
        description: 'Get a $5 discount on any keyboard.',
        promotion_type: 'FixedAmount',
        value: '5.00',
        start_date: '2025-11-01T00:00:00Z',
        end_date: '2025-11-15T23:59:59Z',
        is_active: false,
        product_id: 3,
        created_at: '2025-09-20T11:30:00Z',
        updated_at: '2025-09-22T12:00:00Z',
    },
];

onMounted(() => {
  promotions.value = dummyData.map(p => ({...p, start_date: new Date(p.start_date), end_date: new Date(p.end_date)}));
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

const savePromotion = () => {
  submitted.value = true;
  if (promotion.value.name && promotion.value.promotion_type && promotion.value.value && promotion.value.start_date && promotion.value.end_date) {
    if (promotion.value.id) {
      const index = promotions.value.findIndex(p => p.id === promotion.value.id);
      if (index > -1) {
        promotions.value[index] = { ...promotions.value[index], ...promotion.value };
      }
    } else {
      promotion.value.id = Math.max(...promotions.value.map(p => p.id)) + 1;
      promotion.value.created_at = new Date();
      promotion.value.updated_at = new Date();
      promotions.value.push(promotion.value);
    }
    promotionDialog.value = false;
    promotion.value = {};
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
        accept: () => {
            deletePromotion(promo);
        }
    });
};

const deletePromotion = (promo) => {
    promotions.value = promotions.value.filter(p => p.id !== promo.id);
};

const deleteSelectedPromotions = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected promotions?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            promotions.value = promotions.value.filter(p => !selectedPromotions.value.includes(p));
            selectedPromotions.value = [];
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
          <span class="text-2xl font-semibold">Promotion Management (Admin)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedPromotions" :disabled="!selectedPromotions || !selectedPromotions.length" />
          </template>
        </Toolbar>

        <DataTable v-model:selection="selectedPromotions" :value="promotions" responsiveLayout="scroll">
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
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="promotion.name" required="true" autofocus :class="{'p-invalid': submitted && !promotion.name}" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !promotion.name">Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea id="description" v-model="promotion.description" rows="3" cols="20" fluid />
                <label for="description">Description</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="promotion_type" v-model="promotion.promotion_type" :options="promotionTypes" placeholder="Select a Type" required="true" :class="{'p-invalid': submitted && !promotion.promotion_type}" fluid />
                <label for="promotion_type">Promotion Type</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber id="value" v-model="promotion.value" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" required="true" :class="{'p-invalid': submitted && !promotion.value}" fluid />
                <label for="value">Value</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Calendar id="start_date" v-model="promotion.start_date" required="true" :class="{'p-invalid': submitted && !promotion.start_date}" fluid></Calendar>
                <label for="start_date">Start Date</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Calendar id="end_date" v-model="promotion.end_date" required="true" :class="{'p-invalid': submitted && !promotion.end_date}" fluid></Calendar>
                <label for="end_date">End Date</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber id="product_id" v-model="promotion.product_id" fluid />
                <label for="product_id">Product ID (Optional)</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2 flex items-center">
            <ToggleSwitch v-model="promotion.is_active" inputId="is_active" class="mr-2" />
            <label for="is_active">Active</label>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="savePromotion" />
        </template>
    </Dialog>

  </div>
</template>