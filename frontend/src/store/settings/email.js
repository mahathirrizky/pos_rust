import { defineStore } from 'pinia';
import { merge } from 'lodash';

const defaultState = {
    fromName: '',
    fromEmail: '',
    smtpServer: '',
    smtpPort: 587,
    smtpUsername: '',
    smtpPassword: '',
};

export const useEmailSettingsStore = defineStore('emailSettings', {
  state: () => ({
    settings: { ...defaultState },
  }),
  actions: {
    setSettings(newSettings) {
      this.settings = merge({}, defaultState, newSettings);
    },
  },
});