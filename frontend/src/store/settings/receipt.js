import { defineStore } from 'pinia';
import { merge } from 'lodash';

const defaultState = {
    headerText: '',
    footerText: '',
    showStoreAddress: true,
};

export const useReceiptSettingsStore = defineStore('receiptSettings', {
  state: () => ({
    settings: { ...defaultState },
  }),
  actions: {
    setSettings(newSettings) {
      this.settings = merge({}, defaultState, newSettings);
    },
  },
});