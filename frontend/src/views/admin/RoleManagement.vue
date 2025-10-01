<template>
  <div class="p-4 bg-surface-100 dark:bg-surface-900 min-h-screen">
    <div>
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold text-surface-800 dark:text-surface-100">Role Management</h1>
        <Button v-if="authStore.hasPermission('roles:create')" label="Add New Role" icon="pi pi-plus" @click="showAddRoleModal = true" class="p-button-raised" severity="success" />
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- Roles List -->
        <div class="lg:col-span-1">
          <Card class="shadow-md">
            <template #title>Roles</template>
            <template #content>
              <div v-if="rolesStore.roles.length === 0" class="text-center text-surface-500 dark:text-surface-400 py-4">No roles found.</div>
              <div v-else class="space-y-2">
                <div v-for="role in rolesStore.roles" :key="role.id"
                     @click="onRowSelect({ data: role })"
                     :class="['p-4 rounded-lg cursor-pointer transition-all duration-200', {
                       'bg-blue-100 border-blue-500': selectedRole && selectedRole.id === role.id,
                       'bg-surface-0 dark:bg-surface-800 hover:bg-surface-100 dark:hover:bg-surface-700 border border-surface-200 dark:border-surface-700': !selectedRole || selectedRole.id !== role.id
                     }]">
                  <div class="flex justify-between items-start">
                    <div>
                      <h3 class="font-semibold text-lg text-surface-800 dark:text-surface-100">{{ role.name }}</h3>
                      <p class="text-sm text-surface-600 dark:text-surface-300">{{ role.description }}</p>
                    </div>
                    <div class="flex items-center space-x-1 flex-shrink-0 mt-1">
                      <Button icon="pi pi-pencil" class="p-button-rounded p-button-text p-button-sm" severity="success"
                              @click.stop="editRole(role)"
                              v-tooltip.top="'Edit Role'"
                              :disabled="!authStore.hasPermission('roles:update')" />
                      <Button icon="pi pi-trash" class="p-button-rounded p-button-text p-button-sm" severity="danger"
                              @click.stop="confirmDeleteRole(role)"
                              v-tooltip.top="'Delete Role'"
                              :disabled="!authStore.hasPermission('roles:delete')" />
                    </div>
                  </div>
                </div>
              </div>
            </template>
          </Card>
        </div>

        <!-- Permissions Section -->
        <div class="lg:col-span-2">
          <Card class="shadow-md">
            <template #title>
              <span v-if="selectedRole">Permissions for {{ selectedRole.name }}</span>
              <span v-else>Permissions</span>
            </template>
            <template #content>
              <div v-if="!selectedRole" class="text-center text-surface-500 dark:text-surface-400 py-16">
                <i class="pi pi-info-circle text-4xl mb-2"></i>
                <p>Select a role from the list to view and manage its permissions.</p>
              </div>
              <div v-else>
                <Tabs :value="selectedTab" @update:value="selectedTab = $event" scrollable>
                  <TabList>
                    <Tab v-for="(group, groupName) in groupedPermissions" :key="groupName" :value="groupName">
                      {{ capitalize(groupName.replace('_', ' ')) }}
                    </Tab>
                  </TabList>
                  <TabPanels>
                    <TabPanel v-for="(group, groupName) in groupedPermissions" :key="groupName" :value="groupName">
                      <div class="p-4 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                        <div v-for="permission in group" :key="permission.id" class="flex items-center">
                          <Checkbox :id="'permission-' + permission.id" :value="permission.id" v-model="selectedPermissions" />
                          <label :for="'permission-' + permission.id" class="ml-2 text-surface-700 dark:text-surface-200">{{ capitalize(permission.action) }}</label>
                        </div>
                      </div>
                    </TabPanel>
                  </TabPanels>
                </Tabs>
                <div class="mt-6 text-right">
                  <Button label="Save Permissions" icon="pi pi-save" @click="savePermissions" class="p-button-raised"
                          :disabled="!authStore.hasPermission('permissions:assign')" />
                </div>
              </div>
            </template>
          </Card>
        </div>
      </div>
    </div>

    <!-- Add/Edit Role Dialog -->
    <Dialog :visible="showAddRoleModal" :header="editingRole ? 'Edit Role' : 'Add New Role'" :modal="true" @update:visible="closeModal" class="p-fluid w-full max-w-md">
      <div class="field mt-4">
        <FloatLabel variant="on">
          <InputText id="roleName" v-model="roleForm.name" required autofocus fluid />
          <label for="roleName">Role Name</label>
        </FloatLabel>
      </div>
      <div class="field mt-4">
        <FloatLabel variant="on">
          <Textarea id="roleDescription" v-model="roleForm.description" fluid rows="3" />
          <label for="roleDescription">Description</label>
        </FloatLabel>
      </div>
      <template #footer>
        <Button label="Cancel" icon="pi pi-times" class="p-button-text" severity="secondary" @click="closeModal" />
        <Button label="Save" icon="pi pi-check" severity="success" @click="saveRole" />
      </template>
    </Dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, defineAsyncComponent, watch } from 'vue';
import { useAuthStore } from '../../store/auth';
import { useRolesStore } from '../../store/roles';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";

// PrimeVue Components
const Button = defineAsyncComponent(() => import('primevue/button'));
const Dialog = defineAsyncComponent(() => import('primevue/dialog'));
const InputText = defineAsyncComponent(() => import('primevue/inputtext'));
const Textarea = defineAsyncComponent(() => import('primevue/textarea'));
const Checkbox = defineAsyncComponent(() => import('primevue/checkbox'));
const Card = defineAsyncComponent(() => import('primevue/card'));
const FloatLabel = defineAsyncComponent(() => import('primevue/floatlabel'));
const Tabs = defineAsyncComponent(() => import('primevue/tabs'));
const TabList = defineAsyncComponent(() => import('primevue/tablist'));
const Tab = defineAsyncComponent(() => import('primevue/tab'));
const TabPanels = defineAsyncComponent(() => import('primevue/tabpanels'));
const TabPanel = defineAsyncComponent(() => import('primevue/tabpanel'));

const capitalize = (s) => {
  if (typeof s !== 'string') return ''
  return s.charAt(0).toUpperCase() + s.slice(1)
}

const authStore = useAuthStore();
const rolesStore = useRolesStore();
const confirm = useConfirm();
const toast = useToast();

const showAddRoleModal = ref(false);
const editingRole = ref(null);
const selectedRole = ref(null);
const roleForm = ref({
  id: null,
  name: '',
  description: ''
});
const selectedPermissions = ref([]);
const selectedTab = ref('dashboard'); // Default tab

// Fetch data on component mount
onMounted(() => {
  rolesStore.fetchAllRoles();
  rolesStore.fetchAllPermissions();
});

// Group permissions by their 'group' property
const groupedPermissions = computed(() => {
  if (!rolesStore.permissions) return {};
  return rolesStore.permissions.reduce((acc, permission) => {
    // Derive group and action from the permission name (e.g., "products:create")
    const [group, action] = permission.name.split(':');
    
    // Create a new permission object with the derived properties for the template
    const permissionForTemplate = {
      ...permission,
      group: group || 'general',
      action: action || permission.name, // Fallback to full name if no colon
    };

    if (!acc[permissionForTemplate.group]) {
      acc[permissionForTemplate.group] = [];
    }
    acc[permissionForTemplate.group].push(permissionForTemplate);
    return acc;
  }, {});
});

// Watch for changes in groupedPermissions and set the default tab
watch(groupedPermissions, (newGroups) => {
  const groupKeys = Object.keys(newGroups);
  if (groupKeys.length > 0 && !groupKeys.includes(selectedTab.value)) {
    selectedTab.value = groupKeys[0];
  }
}, { immediate: true });


const originalPermissionsForSelectedRole = ref([]);

// Handle role selection
const onRowSelect = async (event) => {
  selectedRole.value = event.data;
  try {
    const permissionIds = await rolesStore.fetchPermissionsForRole(event.data.id);
    // The backend returns an array of IDs, so we use it directly.
    originalPermissionsForSelectedRole.value = permissionIds;
    selectedPermissions.value = [...permissionIds]; // Use a copy for mutation

    const groupKeys = Object.keys(groupedPermissions.value);
    if (groupKeys.length > 0) {
      selectedTab.value = groupKeys[0];
    }
  } catch (error) {
    console.error(`Failed to fetch permissions for role ${event.data.id}:`, error);
    originalPermissionsForSelectedRole.value = [];
    selectedPermissions.value = [];
  }
};

// Open dialog to edit a role
const editRole = (role) => {
  editingRole.value = role;
  roleForm.value = { ...role };
  showAddRoleModal.value = true;
};

// Close the add/edit dialog
const closeModal = () => {
  showAddRoleModal.value = false;
  editingRole.value = null;
  roleForm.value = { id: null, name: '', description: '' };
};

// Save or update a role
const saveRole = async () => {
  if (editingRole.value) {
    await rolesStore.updateRole(roleForm.value.id, roleForm.value);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Role updated successfully', life: 3000 });
  } else {
    await rolesStore.createRole(roleForm.value);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Role created successfully', life: 3000 });
  }
  closeModal();
};

// Save permissions for the selected role
const savePermissions = async () => {
  console.log("Attempting to save permissions...");
  if (!selectedRole.value) {
    console.log("No role selected.");
    return;
  }

  const originalPermIds = new Set(originalPermissionsForSelectedRole.value);
  const selectedPermIds = new Set(selectedPermissions.value);

  const added = [...selectedPermIds].filter(id => !originalPermIds.has(id));
  const removed = [...originalPermIds].filter(id => !selectedPermIds.has(id));

  console.log("Permissions to add:", added);
  console.log("Permissions to remove:", removed);

  if (added.length === 0 && removed.length === 0) {
    toast.add({ severity: 'info', summary: 'No Changes', detail: 'No permissions were changed.', life: 3000 });
    return;
  }

  try {
    console.log(`Calling store to update role ${selectedRole.value.id} with`, { added, removed });
    await rolesStore.updateRolePermissions(selectedRole.value.id, added, removed);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Permissions updated successfully', life: 3000 });
    
    const updatedPermissionIds = await rolesStore.fetchPermissionsForRole(selectedRole.value.id);
    originalPermissionsForSelectedRole.value = updatedPermissionIds;
    selectedPermissions.value = [...updatedPermissionIds];
    console.log("Permissions updated successfully on the frontend state.");

  } catch (error) {
    console.error('Full error object on savePermissions:', error);
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to update permissions. Check console for details.', life: 3000 });
  }
};

// Confirmation for deleting a role
const confirmDeleteRole = (role) => {
  confirm.require({
    message: `Are you sure you want to delete the role "${role.name}"?`,
    header: 'Delete Confirmation',
    icon: 'pi pi-info-circle',
    acceptClass: 'p-button-danger',
    accept: () => {
      deleteRole(role);
    },
  });
};

// Delete a role
const deleteRole = async (role) => {
  await rolesStore.deleteRole(role.id);
  if (selectedRole.value && selectedRole.value.id === role.id) {
    selectedRole.value = null;
  }
  toast.add({ severity: 'success', summary: 'Deleted', detail: `Role "${role.name}" has been deleted.`, life: 3000 });
};

</script>

