import { defineStore } from 'pinia';
import axios from 'axios';

export const useRolesStore = defineStore('roles', {
  state: () => ({
    roles: [],
    permissions: [],
    selectedRolePermissions: [],
  }),
  actions: {
    async fetchAllRoles() {
      try {
        const response = await axios.get('/api/roles');
        this.roles = response.data.data.filter(role => role.name !== 'Owner' && role.name !== 'Admin');
      } catch (error) {
        console.error('Error fetching roles:', error);
        // Consider adding a toast/notification here if needed
        throw error; // Re-throw to allow component to handle
      }
    },
    async fetchAllPermissions() {
      try {
        const response = await axios.get('/api/permissions');
        this.permissions = response.data.data;
      } catch (error) {
        console.error('Error fetching permissions:', error);
        throw error;
      }
    },
    async fetchPermissionsForRole(roleId) {
      try {
        const response = await axios.get(`/api/roles/${roleId}/permissions`);
        this.selectedRolePermissions = response.data.data;
        return this.selectedRolePermissions;
      } catch (error) {
        console.error(`Error fetching permissions for role ${roleId}:`, error);
        this.selectedRolePermissions = []; // Reset on error
        throw error;
      }
    },
    async createRole(roleData) {
      const response = await axios.post('/api/roles', roleData);
      this.fetchAllRoles(); // Refresh the list
      return response.data.data;
    },

    async updateRole(roleId, roleData) {
      const response = await axios.put(`/api/roles/${roleId}`, roleData);
      this.fetchAllRoles(); // Refresh the list
      return response.data.data;
    },

    async deleteRole(roleId) {
      await axios.delete(`/api/roles/${roleId}`);
      this.fetchAllRoles(); // Refresh the list
    },

    async updateRolePermissions(roleId, added, removed) {
      const promises = [];
      for (const permId of added) {
        promises.push(axios.post('/api/roles/assign-permission', { role_id: roleId, permission_id: permId }));
      }
      for (const permId of removed) {
        promises.push(axios.post('/api/roles/remove-permission', { role_id: roleId, permission_id: permId }));
      }
      await Promise.all(promises);
    }
  },
});
