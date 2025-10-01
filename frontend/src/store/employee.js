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
        console.log('API Response data:', response.data.data);
        this.employees = response.data.data;
        console.log('Store employees after assignment:', this.employees);
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
        const url = isUpdate ? `/api/employees/${employee.id}` : '/api/employees';
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
        try {
            await axios.delete(`/api/employees/${employee.id}`, {
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

    async uploadEmployeeImage(formData) {
        const authStore = useAuthStore();
        if (!authStore.token) throw new Error('Not authenticated');
        try {
            const response = await axios.post('/api/upload/employee', formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            return response.data.url; 
        } catch (error) {
            console.error('Error uploading employee image in store:', error);
            throw error;
        }
    },

    async removeEmployeePhoto(photoUrl) {
        const authStore = useAuthStore();
        if (!authStore.token) throw new Error('Not authenticated');
        try {
            await axios.delete('/api/upload/employee', {
                headers: { Authorization: `Bearer ${authStore.token}` },
                data: { url: photoUrl }
            });
        } catch (error) {
            console.error('Error removing employee photo:', error);
            throw error;
        }
    },
  },
});