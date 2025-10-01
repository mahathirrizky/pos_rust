import { defineStore } from 'pinia';
import { merge } from 'lodash';

const defaultState = {
    siteName: '',
    logoUrl: '',
    defaultTaxRate: 0,
    currencySymbol: '$',
    currencyCode: 'USD',
    defaultDateFormat: 'MM/DD/YYYY',
    enablePromotions: true,
};

export const useGeneralSettingsStore = defineStore('generalSettings', {
  state: () => ({
    settings: { ...defaultState },
  }),
  actions: {
    setSettings(newSettings) {
      this.settings = merge({}, defaultState, newSettings);
    },
  },
});