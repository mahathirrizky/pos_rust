import { defineStore } from 'pinia';
import axios from 'axios';
import { merge } from 'lodash';

// Import individual settings stores
import { useGeneralSettingsStore } from './settings/general';
import { useReceiptSettingsStore } from './settings/receipt';
import { useSecuritySettingsStore } from './settings/security';
import { useIntegrationsSettingsStore } from './settings/integrations';
import { useEmailSettingsStore } from './settings/email';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchSettings() {
      this.isLoading = true;
      this.error = null;

      // Get instances of the individual stores
      const generalStore = useGeneralSettingsStore();
      const receiptStore = useReceiptSettingsStore();
      const securityStore = useSecuritySettingsStore();
      const integrationsStore = useIntegrationsSettingsStore();
      const emailStore = useEmailSettingsStore();

      try {
        // For now, we'll simulate a successful fetch with default data
        // because the API endpoint doesn't exist yet.
        // In the future, this will be: const response = await axios.get('/api/admin/settings');
        const response = { 
          data: { 
            data: { /* This will be filled by the backend */ } 
          }
        };
        
        // Use actions in individual stores to set their state
        generalStore.setSettings(response.data.data.general || {});
        receiptStore.setSettings(response.data.data.receipt || {});
        securityStore.setSettings(response.data.data.security || {});
        integrationsStore.setSettings(response.data.data.integrations || {});
        emailStore.setSettings(response.data.data.email || {});

      } catch (error) {
        this.error = error;
        // Since the endpoint doesn't exist, we'll log a specific error
        console.error('Error fetching settings (endpoint might not exist):', error);
        // We still call the setters with empty objects to ensure the UI has the default state
        generalStore.setSettings({});
        receiptStore.setSettings({});
        securityStore.setSettings({});
        integrationsStore.setSettings({});
        emailStore.setSettings({});
      } finally {
        this.isLoading = false;
      }
    },

    async saveSettings() {
      this.isLoading = true;
      this.error = null;

      // Get instances and their state
      const generalStore = useGeneralSettingsStore();
      const receiptStore = useReceiptSettingsStore();
      const securityStore = useSecuritySettingsStore();
      const integrationsStore = useIntegrationsSettingsStore();
      const emailStore = useEmailSettingsStore();

      const allSettings = {
        general: generalStore.settings,
        receipt: receiptStore.settings,
        security: securityStore.settings,
        integrations: integrationsStore.settings,
        email: emailStore.settings,
      };

      try {
        // The actual API call to save all settings
        await axios.post('/api/admin/settings', allSettings);
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
