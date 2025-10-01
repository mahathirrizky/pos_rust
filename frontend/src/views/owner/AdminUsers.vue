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
import Password from 'primevue/password';
import FloatLabel from 'primevue/floatlabel';
import { useEmployeeStore } from '../../store/employee';

const confirm = useConfirm();
const toast = useToast();
const employeeStore = useEmployeeStore();

const adminUserDialog = ref(false);
const submitted = ref(false);
const adminUser = ref({});

onMounted(() => {
  employeeStore.fetchEmployees({ role: 'Admin' });
});

const openNew = () => {
  adminUser.value = {};
  submitted.value = false;
  adminUserDialog.value = true;
};

const hideDialog = () => {
  adminUserDialog.value = false;
  submitted.value = false;
};

const saveAdminUser = async () => {
  submitted.value = true;
  if (!adminUser.value.first_name || !adminUser.value.email) {
    return;
  }

  if (!adminUser.value.id && !adminUser.value.password) {
      toast.add({ severity: 'error', summary: 'Error', detail: 'Password is required for new users.', life: 3000 });
      return;
  }

  try {
    const isUpdate = !!adminUser.value.id;
    await employeeStore.saveEmployee(adminUser.value);
    
    toast.add({ severity: 'success', summary: 'Success', detail: `Admin user ${isUpdate ? 'updated' : 'created'} successfully.`, life: 3000 });
    adminUserDialog.value = false;
    employeeStore.fetchEmployees({ role: 'Admin' });
  } catch (err) {
      toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const editAdminUser = (user) => {
  adminUser.value = { ...user };
  adminUserDialog.value = true;
};

const confirmDeleteAdminUser = (user) => {
    confirm.require({
        message: `Are you sure you want to delete ${user.first_name}?`,
        header: 'Delete Confirmation',
        icon: 'pi pi-info-circle',
        acceptClass: 'p-button-danger',
        accept: async () => {
            try {
                await employeeStore.deleteEmployee(user);
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Admin user deleted', life: 3000 });
                employeeStore.fetchEmployees({ role: 'Admin' });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
            }
        },
    });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <span class="text-2xl font-semibold">Admin User Management</span>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New Admin" icon="pi pi-plus" class="mr-2" @click="openNew" />
          </template>
        </Toolbar>

        <DataTable :value="employeeStore.employees" responsiveLayout="scroll">
          <Column field="id" header="ID"></Column>
          <Column field="first_name" header="First Name"></Column>
          <Column field="last_name" header="Last Name"></Column>
          <Column field="email" header="Email"></Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="p-button-rounded p-button-success mr-2" @click="editAdminUser(slotProps.data)" />
                <Button icon="pi pi-trash" class="p-button-rounded p-button-danger" @click="confirmDeleteAdminUser(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="adminUserDialog" :style="{width: '800px'}" header="Admin User Details" :modal="true" class="p-fluid">
        <div class="p-grid p-fluid grid-nogutter">
            <div class="field col-6 p-2 mb-4">
                <FloatLabel variant="on">
                    <InputText id="first_name" v-model.trim="adminUser.first_name" required :class="{'p-invalid': submitted && !adminUser.first_name}" variant="filled" fluid />
                    <label for="first_name">First Name</label>
                </FloatLabel>
                <small class="p-error" v-if="submitted && !adminUser.first_name">First Name is required.</small>
            </div>
            <div class="field col-6 p-2 mb-4">
                <FloatLabel variant="on">
                    <InputText id="last_name" v-model.trim="adminUser.last_name" variant="filled" fluid />
                    <label for="last_name">Last Name</label>
                </FloatLabel>
            </div>
            <div class="field col-12 p-2 mb-4">
                <FloatLabel variant="on">
                    <InputText id="email" v-model.trim="adminUser.email" required :class="{'p-invalid': submitted && !adminUser.email}" variant="filled" fluid />
                    <label for="email">Email</label>
                </FloatLabel>
                <small class="p-error" v-if="submitted && !adminUser.email">Email is required.</small>
            </div>
            <div class="field col-12 p-2 mb-4">
                <FloatLabel variant="on">
                    <InputText id="phone" v-model.trim="adminUser.phone" variant="filled" fluid />
                    <label for="phone">Phone</label>
                </FloatLabel>
            </div>
            <div class="field col-12 p-2 mb-4">
                <FloatLabel variant="on">
                    <Password id="password" v-model="adminUser.password" :feedback="false" :class="{'p-invalid': submitted && !adminUser.id && !adminUser.password}" fluid />
                    <label for="password">Password (leave blank to keep current password)</label>
                </FloatLabel>
                <small class="p-error" v-if="submitted && !adminUser.id && !adminUser.password">Password is required for new users.</small>
            </div>
        </div>

        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveAdminUser" />
        </template>
    </Dialog>

  </div>
</template>