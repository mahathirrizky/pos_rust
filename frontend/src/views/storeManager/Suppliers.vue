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
import FloatLabel from 'primevue/floatlabel';

const confirm = useConfirm();
const suppliers = ref([]);
const selectedSuppliers = ref([]);
const supplierDialog = ref(false);
const submitted = ref(false);
const supplier = ref({});

// Dummy data mimicking backend response
const dummyData = [
    {
        id: 1,
        name: 'Tech Supplies Inc.',
        contact_person: 'Alex Green',
        email: 'alex@techsupplies.com',
        phone: '111-222-3333',
        address: '123 Tech Road, Silicon Valley, CA',
        created_at: '2025-01-15T09:30:00Z',
        updated_at: '2025-08-01T14:00:00Z',
    },
    {
        id: 2,
        name: 'Office Gear Co.',
        contact_person: 'Sarah Brown',
        email: 'sarah@officegear.com',
        phone: '444-555-6666',
        address: '456 Office Park, Business City, NY',
        created_at: '2024-11-20T11:00:00Z',
        updated_at: '2025-07-25T10:15:00Z',
    },
];

onMounted(() => {
  suppliers.value = dummyData;
});

const openNew = () => {
  supplier.value = {};
  submitted.value = false;
  supplierDialog.value = true;
};

const hideDialog = () => {
  supplierDialog.value = false;
  submitted.value = false;
};

const saveSupplier = () => {
  submitted.value = true;
  if (supplier.value.name) {
    if (supplier.value.id) {
      const index = suppliers.value.findIndex(s => s.id === supplier.value.id);
      if (index > -1) {
        suppliers.value[index] = { ...suppliers.value[index], ...supplier.value };
      }
    } else {
      supplier.value.id = Math.max(...suppliers.value.map(s => s.id)) + 1;
      supplier.value.created_at = new Date().toISOString();
      supplier.value.updated_at = new Date().toISOString();
      suppliers.value.push(supplier.value);
    }
    supplierDialog.value = false;
    supplier.value = {};
  }
};

const editSupplier = (supp) => {
  supplier.value = { ...supp };
  supplierDialog.value = true;
};

const confirmDeleteSupplier = (supp) => {
    confirm.require({
        message: 'Are you sure you want to delete this supplier?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            deleteSupplier(supp);
        }
    });
};

const deleteSupplier = (supp) => {
    suppliers.value = suppliers.value.filter(s => s.id !== supp.id);
};

const deleteSelectedSuppliers = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected suppliers?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            suppliers.value = suppliers.value.filter(s => !selectedSuppliers.value.includes(s));
            selectedSuppliers.value = [];
        }
    });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Supplier Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedSuppliers" :disabled="!selectedSuppliers || !selectedSuppliers.length" />
          </template>
        </Toolbar>

        <DataTable v-model:selection="selectedSuppliers" :value="suppliers" responsiveLayout="scroll">
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="id" header="ID" :sortable="true"></Column>
          <Column field="name" header="Name" :sortable="true"></Column>
          <Column field="contact_person" header="Contact Person" :sortable="true"></Column>
          <Column field="email" header="Email" :sortable="true"></Column>
          <Column field="phone" header="Phone" :sortable="true"></Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editSupplier(slotProps.data)" />
                <Button icon="pi pi-trash" severity="warning" rounded @click="confirmDeleteSupplier(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="supplierDialog" :style="{width: '450px'}" header="Supplier Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="supplier.name" required="true" autofocus :class="{'p-invalid': submitted && !supplier.name}" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !supplier.name">Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="contact_person" v-model.trim="supplier.contact_person" fluid />
                <label for="contact_person">Contact Person</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="supplier.email" fluid />
                <label for="email">Email</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="phone" v-model.trim="supplier.phone" fluid />
                <label for="phone">Phone</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea id="address" v-model="supplier.address" rows="3" cols="20" fluid />
                <label for="address">Address</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveSupplier" />
        </template>
    </Dialog>

  </div>
</template>
