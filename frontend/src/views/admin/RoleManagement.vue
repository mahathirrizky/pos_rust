<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-4">Role Management</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <!-- Roles Section -->
      <div>
        <h2 class="text-xl font-semibold mb-4">Roles</h2>
        <div class="bg-white p-4 rounded-lg shadow">
          <div class="flex justify-between items-center mb-4">
            <h3 class="text-lg">Existing Roles</h3>
            <Button v-if="authStore.hasPermission('manage_roles')" label="Add Role" icon="pi pi-plus" @click="showAddRoleModal = true" />
          </div>
          <DataTable :value="filteredRoles" selectionMode="single" dataKey="id" @rowSelect="onRowSelect"
                     :paginator="true" :rows="5" responsiveLayout="scroll">
            <Column field="name" header="Role Name"></Column>
            <Column field="description" header="Description"></Column>
            <Column header="Actions">
              <template #body="slotProps">
                <Button icon="pi pi-pencil" class="p-button-rounded p-button-text p-button-sm mr-2"
                        @click="editRole(slotProps.data)"
                        v-tooltip.top="'Edit Role'"
                        :disabled="!authStore.hasPermission('manage_roles') || isProtectedRole(slotProps.data.name)" />
                <Button icon="pi pi-trash" class="p-button-rounded p-button-text p-button-danger p-button-sm"
                        @click="confirmDeleteRole(slotProps.data)"
                        v-tooltip.top="'Delete Role'"
                        :disabled="!authStore.hasPermission('manage_roles') || isProtectedRole(slotProps.data.name)" />
              </template>
            </Column>
          </DataTable>
        </div>
      </div>

      <!-- Permissions Section -->
      <div v-if="authStore.hasPermission('manage_permissions')">
        <h2 class="text-xl font-semibold mb-4">Permissions</h2>
        <div class="bg-white p-4 rounded-lg shadow">
          <div v-if="selectedRole">
            <h3 class="text-lg mb-4">Permissions for {{ selectedRole.name }}</h3>
            <div class="grid grid-cols-2 gap-2">
              <div v-for="permission in permissions" :key="permission.id" class="flex items-center">
                <Checkbox :id="'permission-' + permission.id" :value="permission.id" v-model="selectedRole.permissions" />
                <label :for="'permission-' + permission.id" class="ml-2">{{ permission.name }}</label>
              </div>
            </div>
            <Button label="Save Permissions" icon="pi pi-save" @click="savePermissions" class="mt-4"
                    :disabled="!authStore.hasPermission('manage_permissions') || isProtectedRole(selectedRole.name)" />
          </div>
          <div v-else>
            <p>Select a role to manage its permissions.</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Role Dialog -->
    <Dialog :visible="showAddRoleModal" :header="editingRole ? 'Edit Role' : 'Add New Role'" :modal="true" @update:visible="closeModal" class="p-fluid">
      <div class="field">
        <label for="roleName">Role Name</label>
        <InputText id="roleName" v-model="roleForm.name" required autofocus />
      </div>
      <div class="field mt-4">
        <label for="roleDescription">Description</label>
        <Textarea id="roleDescription" v-model="roleForm.description" rows="3" />
      </div>
      <template #footer>
        <Button label="Cancel" icon="pi pi-times" class="p-button-text" @click="closeModal" />
        <Button label="Save" icon="pi pi-check" @click="saveRole" />
      </template>
    </Dialog>

  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { useAuthStore } from '../../store/auth';
import axios from 'axios';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";

// PrimeVue Components
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import Checkbox from 'primevue/checkbox';

const authStore = useAuthStore();
const confirm = useConfirm();
const toast = useToast();

const roles = ref([]);
const permissions = ref([]);
const selectedRole = ref(null);
const showAddRoleModal = ref(false);
const editingRole = ref(null);
const roleForm = ref({ id: null, name: '', description: null });
const originalRolePermissions = ref([]);

const protectedRoles = ['Owner', 'Admin'];

const filteredRoles = computed(() => {
  return roles.value.filter(role => !protectedRoles.includes(role.name));
});

const isProtectedRole = (roleName) => {
  return protectedRoles.includes(roleName);
};

const fetchRoles = async () => {
  try {
    const response = await axios.get('/api/roles');
    roles.value = response.data.data;
  } catch (error) {
    console.error('Error fetching roles:', error);
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to fetch roles', life: 3000 });
  }
};

const fetchPermissions = async () => {
  try {
    const response = await axios.get('/api/permissions');
    permissions.value = response.data.data;
  } catch (error) {
    console.error('Error fetching permissions:', error);
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to fetch permissions', life: 3000 });
  }
};

const fetchRolePermissions = async (roleId) => {
  try {
    const response = await axios.get(`/api/roles/${roleId}/permissions`);
    // The backend returns an array of permission IDs, convert to string for v-model
    return response.data.data.map(pId => pId);
  } catch (error) {
    console.error(`Error fetching permissions for role ${roleId}:`, error);
    toast.add({ severity: 'error', summary: 'Error', detail: `Failed to fetch permissions for role ${roleId}`, life: 3000 });
    return [];
  }
};

onMounted(() => {
  fetchRoles();
  fetchPermissions();
});

const onRowSelect = async (event) => {
  if (isProtectedRole(event.data.name)) {
    selectedRole.value = null; // Do not select protected roles
    return;
  }
  selectedRole.value = { ...event.data };
  originalRolePermissions.value = await fetchRolePermissions(event.data.id);
  selectedRole.value.permissions = [...originalRolePermissions.value]; // Initialize with current permissions
};

const editRole = async (role) => {
  if (isProtectedRole(role.name)) return; // Prevent editing protected roles
  selectedRole.value = { ...role };
  roleForm.value = { ...role };
  showAddRoleModal.value = true;
  editingRole.value = role;
};

const confirmDeleteRole = (role) => {
  if (isProtectedRole(role.name)) return; // Prevent deleting protected roles
  confirm.require({
    message: `Are you sure you want to delete the role "${role.name}"? This action cannot be undone.`, header: 'Confirmation',
    icon: 'pi pi-exclamation-triangle',
    accept: () => {
      deleteRole(role);
    },
    reject: () => {
      // Callback to execute when action is rejected
    }
  });
};

const deleteRole = async (role) => {
  try {
    await axios.delete(`/api/roles/${role.id}`);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Role deleted successfully', life: 3000 });
    fetchRoles();
    closeModal();
  } catch (error) {
    console.error('Error deleting role:', error);
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete role', life: 3000 });
  }
};

const saveRole = async () => {
  try {
    if (editingRole.value) {
      await axios.put(`/api/roles/${roleForm.value.id}`, roleForm.value);
      toast.add({ severity: 'success', summary: 'Success', detail: 'Role updated successfully', life: 3000 });
    } else {
      await axios.post('/api/roles', roleForm.value);
      toast.add({ severity: 'success', summary: 'Success', detail: 'Role created successfully', life: 3000 });
    }
    closeModal();
    fetchRoles();
  } catch (error) {
    console.error('Error saving role:', error);
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to save role', life: 3000 });
  }
};

const savePermissions = async () => {
  if (!selectedRole.value) return;
  if (isProtectedRole(selectedRole.value.name)) return; // Prevent changing permissions for protected roles

  const currentPermissions = new Set(selectedRole.value.permissions);
  const oldPermissions = new Set(originalRolePermissions.value);

  const addedPermissions = [...currentPermissions].filter(p => !oldPermissions.has(p));
  const removedPermissions = [...oldPermissions].filter(p => !currentPermissions.has(p));

  try {
    for (const permId of addedPermissions) {
      await axios.post('/api/roles/assign-permission', { role_id: selectedRole.value.id, permission_id: permId });
    }
    for (const permId of removedPermissions) {
      await axios.post('/api/roles/remove-permission', { role_id: selectedRole.value.id, permission_id: permId });
    }
    toast.add({ severity: 'success', summary: 'Success', detail: 'Permissions updated successfully!', life: 3000 });
    originalRolePermissions.value = [...selectedRole.value.permissions]; // Update original
  } catch (error) {
    console.error('Error saving permissions:', error);
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to update permissions.', life: 3000 });
  }
};

const closeModal = () => {
  showAddRoleModal.value = false;
  editingRole.value = null;
  roleForm.value = { id: null, name: '', description: null };
  selectedRole.value = null;
};

</script>

<style scoped>
/* You can add component-specific styles here */
</style>


