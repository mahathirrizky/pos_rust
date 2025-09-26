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
import Avatar from 'primevue/avatar';
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
        name: 'Tech Innovators Inc.',
        contact_person: 'Alex Green',
        email: 'alex@techinnovators.com',
        phone: '111-222-3333',
        address: '123 Tech Road, Silicon Valley, CA',
        product_count: 15,
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
        product_count: 32,
        created_at: '2024-11-20T11:00:00Z',
        updated_at: '2025-07-25T10:15:00Z',
    },
    {
        id: 3,
        name: 'Global Foods Distribution',
        contact_person: 'David Chen',
        email: 'david.chen@globalfoods.com',
        phone: '777-888-9999',
        address: '789 Food Plaza, Gourmet Town, TX',
        product_count: 112,
        created_at: '2023-05-10T15:00:00Z',
        updated_at: '2025-09-20T18:00:00Z',
    },
    {
        id: 4,
        name: 'Fashion Forward Fabrics',
        contact_person: 'Maria Garcia',
        email: 'maria@fashionfabrics.com',
        phone: '321-654-9870',
        address: '101 Fashion Ave, Style City, FL',
        product_count: 250,
        created_at: '2024-08-30T09:00:00Z',
        updated_at: '2025-09-15T12:30:00Z',
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
  if (supplier.value.name && supplier.value.email) {
    if (supplier.value.id) {
      const index = suppliers.value.findIndex(s => s.id === supplier.value.id);
      if (index > -1) {
        suppliers.value[index] = { ...suppliers.value[index], ...supplier.value, updated_at: new Date().toISOString() };
      }
    } else {
      supplier.value.id = Math.max(0, ...suppliers.value.map(s => s.id)) + 1;
      supplier.value.product_count = 0;
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
        message: `Are you sure you want to delete the ${selectedSuppliers.value.length} selected suppliers?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            suppliers.value = suppliers.value.filter(s => !selectedSuppliers.value.includes(s));
            selectedSuppliers.value = [];
        }
    });
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric', 
    hour: '2-digit', minute: '2-digit'
  });
};

const getInitials = (name) => {
    if (!name) return '';
    return name.split(' ').map(n => n[0]).join('');
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
            <Button label="Delete Selected" icon="pi pi-trash" severity="danger" @click="deleteSelectedSuppliers" :disabled="!selectedSuppliers || !selectedSuppliers.length" />
          </template>
        </Toolbar>

        <DataTable 
          v-model:selection="selectedSuppliers" 
          :value="suppliers" 
          responsiveLayout="scroll"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            <div class="text-center py-8">
              <i class="pi pi-truck text-4xl text-gray-400 mb-2"></i>
              <h3 class="text-xl font-semibold text-gray-600">No Suppliers Found</h3>
              <p class="text-gray-500">Click the "New" button to add a supplier.</p>
            </div>
          </template>

          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="name" header="Supplier Name" :sortable="true"></Column>
          <Column field="contact_person" header="Contact" :sortable="true">
            <template #body="slotProps">
              <div class="flex items-center">
                <Avatar :label="getInitials(slotProps.data.contact_person)" class="mr-2" shape="circle" />
                <div>
                  <div>{{ slotProps.data.contact_person }}</div>
                  <div class="text-sm text-gray-500">{{ slotProps.data.email }}</div>
                </div>
              </div>
            </template>
          </Column>
          <Column field="phone" header="Phone" :sortable="true"></Column>
          <Column field="product_count" header="Products" :sortable="true" headerStyle="text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <span class="font-medium">{{ slotProps.data.product_count }}</span>
            </template>
          </Column>
          <Column field="updated_at" header="Last Updated" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.updated_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editSupplier(slotProps.data)" v-tooltip.top="'Edit'" />
                <Button icon="pi pi-trash" severity="warning" rounded @click="confirmDeleteSupplier(slotProps.data)" v-tooltip.top="'Delete'" />
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
                <label for="name">Supplier Name</label>
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
                <InputText id="email" v-model.trim="supplier.email" required="true" :class="{'p-invalid': submitted && !supplier.email}" fluid />
                <label for="email">Email</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !supplier.email">Email is required.</small>
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
