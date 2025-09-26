import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useEmployeeStore = defineStore('employee', {
  state: () => ({
    employees: [],
    stores: [],
  }),
  actions: {
    async fetchEmployees(filters = {}) {
      const authStore = useAuthStore();
      if (!authStore.token) {
        console.error('No token found, user is not authenticated');
        return;
      }
      try {
        const params = new URLSearchParams();
        if (filters.store_id) {
            params.append('store_id', filters.store_id);
        }
        if (filters.roles_to_exclude) {
            params.append('roles_to_exclude', filters.roles_to_exclude);
        }
        if (filters.role) {
            params.append('role', filters.role);
        }

        const response = await axios.get('/api/employees', {
          headers: {
            Authorization: `Bearer ${authStore.token}`,
          },
          params,
        });
        this.employees = response.data.data;
      } catch (error) {
        console.error('Error fetching employees:', error);
      }
    },
    async fetchStores() {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        try {
            const response = await axios.get('/api/stores', {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            this.stores = response.data.data.map(s => ({ label: s.name, value: s.id }));
        } catch (error) {
            console.error('Error fetching stores:', error);
        }
    },
    async saveEmployee(employee) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        const isUpdate = !!employee.id;
        const roleEndpoint = employee.role.toLowerCase();
        const url = isUpdate ? `/api/employees/${roleEndpoint}/${employee.id}` : `/api/employees/${roleEndpoint}`;
        const method = isUpdate ? 'put' : 'post';

        try {
            await axios({
                method,
                url,
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
                data: employee,
            });
            await this.fetchEmployees();
        } catch (error) {
            console.error('Error saving employee:', error);
            throw error;
        }
    },
    async deleteEmployee(employee) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        const roleEndpoint = employee.role.toLowerCase();
        try {
            await axios.delete(`/api/employees/${roleEndpoint}/${employee.id}`, {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            await this.fetchEmployees();
        } catch (error) {
            console.error('Error deleting employee:', error);
            throw error;
        }
    },
  },
});