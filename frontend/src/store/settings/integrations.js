import { defineStore } from 'pinia';
import { merge } from 'lodash';

const defaultState = {
    paymentGatewayApiKey: '',
};

export const useIntegrationsSettingsStore = defineStore('integrationsSettings', {
  state: () => ({
    settings: { ...defaultState },
  }),
  actions: {
    setSettings(newSettings) {
      this.settings = merge({}, defaultState, newSettings);
    },
  },
});