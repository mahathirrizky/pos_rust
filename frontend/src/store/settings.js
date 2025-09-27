import { defineStore } from 'pinia';
import axios from 'axios';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    settings: {
        general: {},
        receipt: {},
        security: {},
        integrations: {}
    },
    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchSettings() {
      this.isLoading = true;
      this.error = null;
      try {
        const response = await axios.get('/api/admin/settings');
        this.settings = response.data.data;
      } catch (error) {
        this.error = error;
        console.error('Error fetching settings:', error);
      } finally {
        this.isLoading = false;
      }
    },
    async saveSettings(settings) {
      this.isLoading = true;
      this.error = null;
      try {
        await axios.post('/api/admin/settings', settings);
      } catch (error) {
        this.error = error;
        console.error('Error saving settings:', error);
        throw error;
      } finally {
        this.isLoading = false;
      }
    },
  },
});
