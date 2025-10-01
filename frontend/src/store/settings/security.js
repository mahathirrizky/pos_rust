import { defineStore } from 'pinia';
import { merge } from 'lodash';

const defaultState = {
    sessionTimeout: 30,
    enable2FA: false,
    passwordPolicy: {
        minLength: 8,
        requireUppercase: true,
        requireLowercase: true,
        requireNumbers: true,
        requireSymbols: false,
    },
    accountLockout: {
        maxFailedAttempts: 5,
        lockoutDuration: 15,
    },
};

export const useSecuritySettingsStore = defineStore('securitySettings', {
  state: () => ({
    settings: { ...defaultState },
  }),
  actions: {
    setSettings(newSettings) {
      this.settings = merge({}, defaultState, newSettings);
    },
  },
});