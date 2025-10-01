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
import FloatLabel from 'primevue/floatlabel';
import { useStoreStore } from '../../store/store';

const confirm = useConfirm();
const toast = useToast();
const storeStore = useStoreStore();

const selectedStores = ref([]);
const storeDialog = ref(false);
const submitted = ref(false);
const store = ref({});

onMounted(() => {
  storeStore.fetchStores();
});

const openNew = () => {
  store.value = {};
  submitted.value = false;
  storeDialog.value = true;
};

const hideDialog = () => {
  storeDialog.value = false;
  submitted.value = false;
};

const saveStore = async () => {
  submitted.value = true;
  if (!store.value.name) {
    return;
  }

  try {
    await storeStore.saveStore(store.value);
    const isUpdate = !!store.value.id;
    toast.add({ severity: 'success', summary: 'Success', detail: `Store ${isUpdate ? 'updated' : 'created'} successfully.`, life: 3000 });
    storeDialog.value = false;
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const editStore = (str) => {
  store.value = { ...str };
  storeDialog.value = true;
};

const confirmDeleteStore = (str) => {
    confirm.require({
        message: 'Are you sure you want to delete this store?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await storeStore.deleteStore(str.id);
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Store deleted', life: 3000 });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
            }
        },
    });
};

const deleteSelectedStores = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedStores.value.length} selected stores?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedStores.value.map(s => storeStore.deleteStore(s.id)));
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Selected stores deleted', life: 3000 });
                selectedStores.value = [];
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete selected stores.', life: 3000 });
            }
        },
    });
};

const formatDate = (value) => {
  if (!value) return '';
  return new Date(value).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit'
  });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Store Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete Selected" icon="pi pi-trash" severity="danger" @click="deleteSelectedStores" :disabled="!selectedStores || !selectedStores.length" />
          </template>
        </Toolbar>

        <DataTable
          v-model:selection="selectedStores"
          :value="storeStore.stores"
          responsiveLayout="scroll"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            <div class="text-center py-8">
              <i class="pi pi-building text-4xl text-surface-400 dark:text-surface-500 mb-2"></i>
              <h3 class="text-xl font-semibold text-surface-600 dark:text-surface-300">No Stores Found</h3>
              <p class="text-surface-500 dark:text-surface-400">Your database is empty. Click the "New" button to add a store.</p>
            </div>
          </template>

          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="name" header="Store Name" :sortable="true"></Column>
          <Column field="address" header="Address" :sortable="true"></Column>
          <Column field="phone" header="Phone" :sortable="true"></Column>
          <Column field="created_at" header="Created At" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.created_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editStore(slotProps.data)" v-tooltip.top="'Edit'" />
                <Button icon="pi pi-trash" severity="danger" rounded @click="confirmDeleteStore(slotProps.data)" v-tooltip.top="'Delete'" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="storeDialog" :style="{width: '450px'}" header="Store Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-6 p-2">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="store.name" required="true" autofocus :class="{'p-invalid': submitted && !store.name}" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !store.name">Name is required.</small>
        </div>
        <div class="field col-6 p-2">
            <FloatLabel variant="on">
                <Textarea id="address" v-model="store.address" rows="3" cols="20" fluid />
                <label for="address">Address</label>
            </FloatLabel>
        </div>
        <div class="field col-6 p-2">
            <FloatLabel variant="on">
                <InputText id="phone" v-model.trim="store.phone" fluid />
                <label for="phone">Phone</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveStore" />
        </template>
    </Dialog>

  </div>
</template>