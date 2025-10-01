<script setup>
import { onMounted, ref } from 'vue';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import InputNumber from 'primevue/inputnumber';
import ToggleSwitch from 'primevue/toggleswitch';
import Password from 'primevue/password';
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import FloatLabel from 'primevue/floatlabel';
import FileUpload from 'primevue/fileupload';
import Select from 'primevue/select';
import { useToast } from 'primevue/usetoast';
import { storeToRefs } from 'pinia';

// Main stores
import { useThemeStore } from '../../store/theme';
import { useSettingsStore } from '../../store/settings';

// Individual settings stores for state access
import { useGeneralSettingsStore } from '../../store/settings/general';
import { useReceiptSettingsStore } from '../../store/settings/receipt';
import { useSecuritySettingsStore } from '../../store/settings/security';
import { useIntegrationsSettingsStore } from '../../store/settings/integrations';
import { useEmailSettingsStore } from '../../store/settings/email';

const toast = useToast();

// --- Main Stores --- 
const themeStore = useThemeStore();
const { isDark } = storeToRefs(themeStore);

// The main settings store for actions (fetch/save)
const settingsStore = useSettingsStore();

// --- Individual Stores for State --- 
// We get the state directly from the individual stores
const { settings: generalSettings } = storeToRefs(useGeneralSettingsStore());
const { settings: receiptSettings } = storeToRefs(useReceiptSettingsStore());
const { settings: securitySettings } = storeToRefs(useSecuritySettingsStore());
const { settings: integrationsSettings } = storeToRefs(useIntegrationsSettingsStore());
const { settings: emailSettings } = storeToRefs(useEmailSettingsStore());


const dateFormats = ref([
    { label: 'DD/MM/YYYY', value: 'DD/MM/YYYY' },
    { label: 'MM/DD/YYYY', value: 'MM/DD/YYYY' },
    { label: 'YYYY-MM-DD', value: 'YYYY-MM-DD' },
    { label: 'Month D, YYYY', value: 'MMMM D, YYYY' },
]);

onMounted(() => {
  // Fetch all settings using the main store
  settingsStore.fetchSettings();
});

const saveSettings = async () => {
    try {
        await settingsStore.saveSettings();
        toast.add({ severity: 'success', summary: 'Success', detail: 'All settings saved successfully', life: 3000 });
    } catch (error) {
        toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to save settings', life: 3000 });
    }
};

const onLogoUpload = (event) => {
    const file = event.files[0];
    console.log('Uploading logo:', file);
    generalSettings.value.logoUrl = 'https://placehold.co/200x100'; // Placeholder URL
    toast.add({ severity: 'info', summary: 'Info', detail: 'Logo upload handler not implemented yet.', life: 3000 });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Global Settings</span>
        </div>
      </template>
      <template #content>
        <Tabs value="0">
          <TabList>
            <Tab value="0">General</Tab>
            <Tab value="1">Receipt</Tab>
            <Tab value="2">Security</Tab>
            <Tab value="3">Integrations</Tab>
            <Tab value="4">Email</Tab>
          </TabList>
          <TabPanels>
            <TabPanel value="0">
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-4">General Application Settings</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="siteName" v-model="generalSettings.siteName" fluid />
                        <label for="siteName" class="font-medium mb-2">Site Name</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <label for="logo-upload" class="font-medium mb-2">Site Logo</label>
                    <div class="flex items-center">
                        <img :src="generalSettings.logoUrl || 'https://placehold.co/200x100'" alt="Current Logo" class="w-32 h-auto mr-4 border rounded-md p-2" />
                        <FileUpload name="logo" @uploader="onLogoUpload" :multiple="false" accept="image/*" :maxFileSize="1000000" customUpload>
                            <template #empty>
                                <p>Drag and drop files to here to upload.</p>
                            </template>
                        </FileUpload>
                    </div>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputNumber v-model="generalSettings.defaultTaxRate" inputId="taxRate" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" fluid />
                        <label for="taxRate" class="font-medium mb-2">Default Tax Rate (%)</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="currencySymbol" v-model="generalSettings.currencySymbol" fluid />
                        <label for="currencySymbol" class="font-medium mb-2">Currency Symbol</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="currencyCode" v-model="generalSettings.currencyCode" fluid />
                        <label for="currencyCode" class="font-medium mb-2">Currency Code (e.g., USD, EUR)</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <Select id="dateFormat" v-model="generalSettings.defaultDateFormat" :options="dateFormats" optionLabel="label" optionValue="value" placeholder="Select a Date Format" fluid></Select>
                        <label for="dateFormat" class="font-medium mb-2">Default Date Format</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="generalSettings.enablePromotions" inputId="enablePromotions" class="mr-2" />
                    <label for="enablePromotions" class="font-medium">Enable Promotions Module</label>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="isDark" inputId="theme-switcher" class="mr-2" />
                    <label for="theme-switcher" class="font-medium">Dark Mode</label>
                  </div>
                </div>
              </div>
            </TabPanel>
            <TabPanel value="1">
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-4">Customer Receipt Settings</h3>
                <div class="grid grid-cols-1 gap-8">
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="receiptHeader" v-model="receiptSettings.headerText" fluid />
                        <label for="receiptHeader" class="font-medium mb-2">Receipt Header Text</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="receiptFooter" v-model="receiptSettings.footerText" fluid />
                        <label for="receiptFooter" class="font-medium mb-2">Receipt Footer Text</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="receiptSettings.showStoreAddress" inputId="showStoreAddress" class="mr-2" />
                    <label for="showStoreAddress" class="font-medium">Show Store Address on Receipt</label>
                  </div>
                </div>
              </div>
            </TabPanel>
            <TabPanel value="2">
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-4">Security Settings</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputNumber v-model="securitySettings.sessionTimeout" inputId="sessionTimeout" fluid />
                        <label for="sessionTimeout" class="font-medium mb-2">Session Timeout (minutes)</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="securitySettings.enable2FA" inputId="enable2FA" class="mr-2" />
                    <label for="enable2FA" class="font-medium">Enable Two-Factor Authentication (2FA) for all users</label>
                  </div>
                </div>

                <div class="mt-8">
                    <h4 class="text-lg font-semibold mb-4 border-t pt-4">Password Policy</h4>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <InputNumber v-model="securitySettings.passwordPolicy.minLength" inputId="minLength" fluid />
                                <label for="minLength" class="font-medium mb-2">Minimum Password Length</label>
                            </FloatLabel>
                        </div>
                        <div class="field flex items-center mt-4">
                            <ToggleSwitch v-model="securitySettings.passwordPolicy.requireUppercase" inputId="requireUppercase" class="mr-2" />
                            <label for="requireUppercase" class="font-medium">Require Uppercase</label>
                        </div>
                        <div class="field flex items-center mt-4">
                            <ToggleSwitch v-model="securitySettings.passwordPolicy.requireLowercase" inputId="requireLowercase" class="mr-2" />
                            <label for="requireLowercase" class="font-medium">Require Lowercase</label>
                        </div>
                        <div class="field flex items-center mt-4">
                            <ToggleSwitch v-model="securitySettings.passwordPolicy.requireNumbers" inputId="requireNumbers" class="mr-2" />
                            <label for="requireNumbers" class="font-medium">Require Numbers</label>
                        </div>
                        <div class="field flex items-center mt-4">
                            <ToggleSwitch v-model="securitySettings.passwordPolicy.requireSymbols" inputId="requireSymbols" class="mr-2" />
                            <label for="requireSymbols" class="font-medium">Require Symbols</label>
                        </div>
                    </div>
                </div>

                <div class="mt-8">
                    <h4 class="text-lg font-semibold mb-4 border-t pt-4">Account Lockout</h4>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <InputNumber v-model="securitySettings.accountLockout.maxFailedAttempts" inputId="maxFailedAttempts" fluid />
                                <label for="maxFailedAttempts" class="font-medium mb-2">Max Failed Login Attempts</label>
                            </FloatLabel>
                        </div>
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <InputNumber v-model="securitySettings.accountLockout.lockoutDuration" inputId="lockoutDuration" fluid />
                                <label for="lockoutDuration" class="font-medium mb-2">Lockout Duration (minutes)</label>
                            </FloatLabel>
                        </div>
                    </div>
                </div>
              </div>
            </TabPanel>
            <TabPanel value="3">
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-4">Third-Party Integrations</h3>
                <div class="grid grid-cols-1 gap-8">
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <Password id="paymentApiKey" v-model="integrationsSettings.paymentGatewayApiKey" :feedback="false" toggleMask fluid />
                        <label for="paymentApiKey" class="font-medium mb-2">Payment Gateway API Key</label>
                    </FloatLabel>
                    <small class="text-surface-500 dark:text-surface-400 mt-1">Enter the API key for your payment processor (e.g., Stripe, PayPal).</small>
                  </div>
                </div>
              </div>
            </TabPanel>
            <TabPanel value="4">
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-4">Email Settings</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="fromName" v-model="emailSettings.fromName" fluid />
                        <label for="fromName" class="font-medium mb-2">From Name</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="fromEmail" v-model="emailSettings.fromEmail" fluid />
                        <label for="fromEmail" class="font-medium mb-2">From Email</label>
                    </FloatLabel>
                  </div>
                </div>

                <div class="mt-8">
                    <h4 class="text-lg font-semibold mb-4 border-t pt-4">SMTP Settings</h4>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <InputText id="smtpServer" v-model="emailSettings.smtpServer" fluid />
                                <label for="smtpServer" class="font-medium mb-2">SMTP Server</label>
                            </FloatLabel>
                        </div>
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <InputNumber v-model="emailSettings.smtpPort" inputId="smtpPort" fluid />
                                <label for="smtpPort" class="font-medium mb-2">SMTP Port</label>
                            </FloatLabel>
                        </div>
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <InputText id="smtpUsername" v-model="emailSettings.smtpUsername" fluid />
                                <label for="smtpUsername" class="font-medium mb-2">SMTP Username</label>
                            </FloatLabel>
                        </div>
                        <div class="field flex flex-col">
                            <FloatLabel variant="on">
                                <Password id="smtpPassword" v-model="emailSettings.smtpPassword" :feedback="false" toggleMask fluid />
                                <label for="smtpPassword" class="font-medium mb-2">SMTP Password</label>
                            </FloatLabel>
                        </div>
                    </div>
                </div>
              </div>
            </TabPanel>
          </TabPanels>
        </Tabs>

        <Toolbar class="mt-4">
            <template #end>
                <Button label="Save Settings" icon="pi pi-check" severity="success" @click="saveSettings" />
            </template>
        </Toolbar>
      </template>
    </Card>
  </div>
</template>