<script setup>
import { ref, onMounted, watch, defineAsyncComponent, computed } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import { storeToRefs } from 'pinia';

import { useEmployeeStore } from '../../store/employee';
import { useRolesStore } from '../../store/roles';

// PrimeVue Components
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import Select from 'primevue/select';
import FloatLabel from 'primevue/floatlabel';
import FileUpload from 'primevue/fileupload';
import ProgressBar from 'primevue/progressbar';
import Badge from 'primevue/badge';
import ConfirmDialog from 'primevue/confirmdialog';

import { usePrimeVue } from 'primevue/config';

const $primevue = usePrimeVue();
const confirm = useConfirm();
const toast = useToast();
const employeeStore = useEmployeeStore();
const { employees } = storeToRefs(employeeStore);
const rolesStore = useRolesStore();

const selectedStoreId = ref(null);
const selectedEmployees = ref([]);
const employeeDialog = ref(false);
const editEmployeeDialog = ref(false);
const submitted = ref(false);
const employee = ref({});

// Helper function to convert URL to File object
const urlToFile = async (url, filename, mimeType) => {
    const res = await fetch(url);
    const blob = await res.blob();
    return new File([blob], filename, { type: mimeType });
}

const formatSize = (bytes) => {
    const k = 1024;
    const dm = 3;
    const sizes = $primevue.config.locale.fileSizeTypes;

    if (bytes === 0) {
        return `0 ${sizes[0]}`;
    }

    const i = Math.floor(Math.log(bytes) / Math.log(k));
    const formattedSize = parseFloat((bytes / Math.pow(k, i)).toFixed(dm));

    return `${formattedSize} ${sizes[i]}`;
};


const uploader = ref(null);
const totalSize = ref(0);
const totalSizePercent = ref(0);
const files = ref([]); 
const newlyUploadedPhotoUrl = ref(null);
const originalPhotoUrl = ref(null);

onMounted(() => {
  employeeStore.fetchEmployees({ roles_to_exclude: 'Owner,Admin' });
  employeeStore.fetchStores();
  rolesStore.fetchAllRoles();
  console.log('Employees in store after onMounted fetch:', employeeStore.employees);
});



const openNew = () => {
  employee.value = {};
  submitted.value = false;
  files.value = [];
  totalSize.value = 0;
  totalSizePercent.value = 0;
  newlyUploadedPhotoUrl.value = null;
  employeeDialog.value = true;
};

const hideDialog = () => { // For Create Dialog
  employeeDialog.value = false;
  submitted.value = false;
  if (newlyUploadedPhotoUrl.value) {
    employeeStore.removeEmployeePhoto(newlyUploadedPhotoUrl.value);
  }
  files.value = [];
  totalSize.value = 0;
  totalSizePercent.value = 0;
  newlyUploadedPhotoUrl.value = null;
};

const saveEmployee = async () => { // For Create Dialog
  submitted.value = true;
  if (!employee.value.first_name || !employee.value.email || !employee.value.role_id) {
    return;
  }

  if (!employee.value.id && !employee.value.password) { // Password required for new employee
      return;
  }

  try {
    const payload = { ...employee.value };
    if (payload.password) {
        payload.password_hash = payload.password;
        delete payload.password;
    }
    await employeeStore.saveEmployee(payload);
    employeeStore.fetchEmployees({ roles_to_exclude: 'Owner,Admin' });
    console.log('Employees in store after save and refetch:', employeeStore.employees);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Employee created successfully.', life: 3000 });
    employeeDialog.value = false;
    files.value = [];
    totalSize.value = 0;
    totalSizePercent.value = 0;
    newlyUploadedPhotoUrl.value = null;
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const hideEditDialog = () => {
  editEmployeeDialog.value = false;
  submitted.value = false;
  if (newlyUploadedPhotoUrl.value) {
    employeeStore.removeEmployeePhoto(newlyUploadedPhotoUrl.value);
  }
  newlyUploadedPhotoUrl.value = null;
  originalPhotoUrl.value = null;
};

const updateEmployee = async () => {
  submitted.value = true;
  if (!employee.value.first_name || !employee.value.email || !employee.value.role_id) {
    return;
  }

  try {
    await employeeStore.saveEmployee(employee.value);
    employeeStore.fetchEmployees({ roles_to_exclude: 'Owner,Admin' });
    toast.add({ severity: 'success', summary: 'Success', detail: 'Employee updated successfully.', life: 3000 });
    
    if (newlyUploadedPhotoUrl.value && originalPhotoUrl.value && newlyUploadedPhotoUrl.value !== originalPhotoUrl.value) {
        await employeeStore.removeEmployeePhoto(originalPhotoUrl.value);
    }

    editEmployeeDialog.value = false;
    newlyUploadedPhotoUrl.value = null;
    originalPhotoUrl.value = null;

  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const editEmployee = (emp) => {
  employee.value = { ...emp };
  submitted.value = false;
  originalPhotoUrl.value = emp.photo_url;
  newlyUploadedPhotoUrl.value = null;
  editEmployeeDialog.value = true;
};

const confirmDeleteEmployee = (emp) => {
    confirm.require({
        message: 'Are you sure you want to delete this employee?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                // Delete the photo first if it exists
                if (emp.photo_url) {
                    await employeeStore.removeEmployeePhoto(emp.photo_url);
                }
                await employeeStore.deleteEmployee(emp);
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Employee deleted', life: 3000 });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
            }
        },
    });
};

const deleteSelectedEmployees = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected employees?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedEmployees.value.map(async (emp) => {
                    if (emp.photo_url) {
                        await employeeStore.removeEmployeePhoto(emp.photo_url);
                    }
                    return employeeStore.deleteEmployee(emp);
                }));
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Selected employees deleted', life: 3000 });
                selectedEmployees.value = [];
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete selected employees.', life: 3000 });
            }
        },
    });
};

const formatDate = (value) => {
  if (!value) return '';
  return new Date(value).toLocaleDateString('en-US', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  });
};

const onSelectedFiles = (event) => {
    const newFiles = event.files;
    files.value = [...files.value, ...newFiles];
    newFiles.forEach((file) => {
        totalSize.value += file.size;
    });
    // Auto-upload
    if (uploader.value) {
        uploader.value.upload();
    }
};

const onRemoveTemplatingFile = (file, removeFileCallback, index) => {
    removeFileCallback(index);
    totalSize.value -= file.size;
    totalSizePercent.value = totalSize.value / 10; // Assuming 1Mb max
};

const onUploadSuccess = (event) => {
  const response = JSON.parse(event.xhr.response);
  employee.value.photo_url = response.url;
  newlyUploadedPhotoUrl.value = response.url;
  files.value = []; // Clear pending files
  totalSize.value = 0;
  totalSizePercent.value = 0;
  toast.add({ severity: "info", summary: "Success", detail: "File Uploaded", life: 3000 });
};

const onUploadError = () => {
    toast.add({ severity: 'error', summary: 'Error', detail: 'File upload failed', life: 3000 });
};

const onUpdatePhoto = (event) => {
  const response = JSON.parse(event.xhr.response);
  const newUrl = response.url;
  employee.value.photo_url = newUrl; // Update the preview
  newlyUploadedPhotoUrl.value = newUrl; // Track for rollback and old file deletion
  toast.add({ severity: 'info', summary: 'Success', detail: 'New photo uploaded. Save the employee to apply changes.', life: 4000 });
};

const onRemoveUploadedFile = async (event) => {
    // This event is tricky with the custom template.
    // We assume it refers to the currently set photo_url.
    if (employee.value.photo_url) {
        try {
            await employeeStore.removeEmployeePhoto(employee.value.photo_url);
            toast.add({ severity: 'success', summary: 'Success', detail: 'Photo removed', life: 3000 });
            employee.value.photo_url = null;
            newlyUploadedPhotoUrl.value = null;
        } catch (error) {
            toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to remove photo', life: 3000 });
        }
    }
};

</script>

<template>
  <div>
    <ConfirmDialog></ConfirmDialog>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Employee Management (Admin)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedEmployees" :disabled="!selectedEmployees || !selectedEmployees.length" />
          </template>
          
        </Toolbar>

        <DataTable
          v-model:selection="selectedEmployees"
          :value="employees"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            No employees found.
          </template>
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="id" header="ID"></Column>
          <Column field="first_name" header="First Name"></Column>
          <Column field="last_name" header="Last Name"></Column>
          <Column field="email" header="Email"></Column>
          <Column field="role" header="Role"></Column>
          <Column field="store_id" header="Store ID"></Column>
          <Column field="created_at" header="Created At">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.created_at) }}
            </template>
          </Column>
          <Column header="Photo">
            <template #body="slotProps">
              <img :src="slotProps.data.photo_url || '/placeholder.svg'" :alt="slotProps.data.first_name" class="w-8 h-8 rounded-full object-cover" />
            </template>
          </Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" @click="editEmployee(slotProps.data)" />
                <Button icon="pi pi-trash" severity="danger" @click="confirmDeleteEmployee(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="employeeDialog" :style="{width: '450px'}" header="Employee Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="firstname" v-model.trim="employee.first_name" required autofocus fluid />
                <label for="firstname">First Name</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="lastname" v-model.trim="employee.last_name" required fluid />
                <label for="lastname">Last Name</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="employee.email" required fluid />
                <label for="email">Email</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="role" v-model="employee.role_id" :options="rolesStore.roles" optionLabel="name" optionValue="id" required fluid />
                <label for="role">Role</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="store" v-model="employee.store_id" :options="employeeStore.stores.filter(s => s.value !== null)" optionLabel="label" optionValue="value"  required fluid />
                <label for="store">Store</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Password id="password" v-model="employee.password" :feedback="false" fluid />
                <label for="password">Password</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FileUpload ref="uploader" name="demo[]" url="/api/upload/employee" @upload="onUploadSuccess($event)" :multiple="false" accept="image/*" :maxFileSize="1000000" @select="onSelectedFiles" @remove-uploaded-file="onRemoveUploadedFile">
                <template #header="{ chooseCallback, files }">
                    <div class="flex flex-wrap justify-between items-center flex-1 gap-4">
                        <div class="flex gap-2">
                            <Button @click="chooseCallback()" label="Pilih Gambar" variant="outlined" severity="secondary"></Button>
                        </div>
                        <ProgressBar :value="totalSizePercent" :showValue="false" class="md:w-20rem h-1 w-full md:ml-auto">
                            <span class="whitespace-nowrap">{{ totalSize }}B / 1Mb</span>
                        </ProgressBar>
                    </div>
                </template>
                <template #content="{ files, uploadedFiles, removeUploadedFileCallback, removeFileCallback }">
                    <div class="flex flex-col gap-8 pt-4">
                        <div v-if="files.length > 0">
                            <h5>Pending</h5>
                            <div class="flex flex-wrap gap-4">
                                <div v-for="(file, index) of files" :key="file.name + file.type + file.size" class="p-8 rounded-border flex flex-col border border-surface items-center gap-4">
                                    <div>
                                        <img role="presentation" :alt="file.name" :src="file.objectURL" width="100" height="50" />
                                    </div>
                                    <span class="font-semibold text-ellipsis max-w-60 whitespace-nowrap overflow-hidden">{{ file.name }}</span>
                                    <div>{{ formatSize(file.size) }}</div>
                                    <Badge value="Pending" severity="warn" />
                                    <Button icon="pi pi-times" @click="onRemoveTemplatingFile(file, removeFileCallback, index)" variant="outlined" rounded severity="danger" />
                                </div>
                            </div>
                        </div>

                        <div v-if="uploadedFiles.length > 0">
                            <h5>Completed</h5>
                            <div class="flex flex-wrap gap-4">
                                <div v-for="(file, index) of uploadedFiles" :key="file.name + file.type + file.size" class="p-8 rounded-border flex flex-col border border-surface items-center gap-4">
                                    <div>
                                        <img role="presentation" :alt="file.name" :src="file.objectURL" width="100" height="50" />
                                    </div>
                                    <span class="font-semibold text-ellipsis max-w-60 whitespace-nowrap overflow-hidden">{{ file.name }}</span>
                                    <div>{{ formatSize(file.size) }}</div>
                                    <Badge value="Completed" class="mt-4" severity="success" />
                                    <Button icon="pi pi-times" @click="removeUploadedFileCallback(index)" variant="outlined" rounded severity="danger" />
                                </div>
                            </div>
                        </div>
                    </div>
                </template>
                <template #empty>
                    <div class="flex items-center justify-center flex-col">
                        <i class="pi pi-cloud-upload !border-2 !rounded-full !p-4 !text-4xl !text-muted-color" />
                        <p class="mt-3 mb-0">Drag and drop files to here to upload.</p>
                    </div>
                </template>
            </FileUpload>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveEmployee" />
        </template>
    </Dialog>

    <Dialog v-model:visible="editEmployeeDialog" :style="{width: '450px'}" header="Edit Employee Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="firstname" v-model.trim="employee.first_name" required autofocus fluid />
                <label for="firstname">First Name</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="lastname" v-model.trim="employee.last_name" required fluid />
                <label for="lastname">Last Name</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="employee.email" required fluid />
                <label for="email">Email</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="role" v-model="employee.role_id" :options="rolesStore.roles" optionLabel="name" optionValue="id" required fluid />
                <label for="role">Role</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="store" v-model="employee.store_id" :options="employeeStore.stores.filter(s => s.value !== null)" optionLabel="label" optionValue="value"  required fluid />
                <label for="store">Store</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Password id="password" v-model="employee.password" :feedback="false" fluid />
                <label for="password">Password</label>
            </FloatLabel>
        </div>
        
        <!-- Photo section for Edit Dialog -->
        <div class="field col-12 p-2">
            <label>Photo</label>
            <div v-if="employee.photo_url" class="mt-2">
                <img :src="employee.photo_url" :alt="employee.first_name" class="w-full h-auto rounded max-w-xs" />
            </div>
            <div v-else class="mt-2">
                <p>No photo available.</p>
            </div>
        </div>

        <div class="field col-12 p-2">
            <label>{{ employee.photo_url ? 'Change Photo' : 'Upload Photo' }}</label>
            <FileUpload 
                name="file" 
                url="/api/upload/employee" 
                @upload="onUpdatePhoto($event)" 
                @error="onUploadError"
                :multiple="false" 
                accept="image/*" 
                :maxFileSize="1000000"
                :auto="true"
                class="mt-2"
            >
                <template #header="{ chooseCallback }">
                    <Button @click="chooseCallback()" label="Pilih Gambar" variant="outlined" severity="secondary"></Button>
                </template>
                <template #empty>
                    <div class="flex items-center justify-center flex-col">
                        <i class="pi pi-cloud-upload !border-2 !rounded-full !p-4 !text-4xl !text-muted-color" />
                        <p class="mt-3 mb-0">Drag and drop files to here to upload.</p>
                    </div>
                </template>
            </FileUpload>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideEditDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="updateEmployee" />
        </template>
    </Dialog>

  </div>
</template>