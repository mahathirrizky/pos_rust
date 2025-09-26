<script setup>
import { ref, onMounted } from 'vue';
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

const settings = ref({
    general: {},
    receipt: {},
    security: {},
    integrations: {}
});

// Dummy data mimicking settings loaded from a backend
const dummySettings = {
    general: {
        defaultTaxRate: 7.5,
        currencySymbol: '',
        currencyCode: 'USD',
        enablePromotions: true,
    },
    receipt: {
        headerText: 'Thank You for Shopping With Us!',
        footerText: 'Please come again. Find us at www.example.com',
        showStoreAddress: true,
    },
    security: {
        sessionTimeout: 30, // in minutes
        enable2FA: false,
    },
    integrations: {
        paymentGatewayApiKey: 'pk_test_1234567890abcdefghij',
    }
};

onMounted(() => {
  // Deep copy to avoid modifying the original dummy data directly
  settings.value = JSON.parse(JSON.stringify(dummySettings));
});

const saveSettings = () => {
    // In a real app, this would send the settings.value to the backend
    console.log("Saving settings:", settings.value);
    alert('Settings saved successfully! (Simulated)');
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
          </TabList>
          <TabPanels>
            <TabPanel value="0">
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-4">General Application Settings</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputNumber v-model="settings.general.defaultTaxRate" inputId="taxRate" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" fluid />
                        <label for="taxRate" class="font-medium mb-2">Default Tax Rate (%)</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="currencySymbol" v-model="settings.general.currencySymbol" fluid />
                        <label for="currencySymbol" class="font-medium mb-2">Currency Symbol</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="currencyCode" v-model="settings.general.currencyCode" fluid />
                        <label for="currencyCode" class="font-medium mb-2">Currency Code (e.g., USD, EUR)</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="settings.general.enablePromotions" inputId="enablePromotions" class="mr-2" />
                    <label for="enablePromotions" class="font-medium">Enable Promotions Module</label>
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
                        <InputText id="receiptHeader" v-model="settings.receipt.headerText" fluid />
                        <label for="receiptHeader" class="font-medium mb-2">Receipt Header Text</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex flex-col">
                    <FloatLabel variant="on">
                        <InputText id="receiptFooter" v-model="settings.receipt.footerText" fluid />
                        <label for="receiptFooter" class="font-medium mb-2">Receipt Footer Text</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="settings.receipt.showStoreAddress" inputId="showStoreAddress" class="mr-2" />
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
                        <InputNumber v-model="settings.security.sessionTimeout" inputId="sessionTimeout" fluid />
                        <label for="sessionTimeout" class="font-medium mb-2">Session Timeout (minutes)</label>
                    </FloatLabel>
                  </div>
                  <div class="field flex items-center mt-4">
                    <ToggleSwitch v-model="settings.security.enable2FA" inputId="enable2FA" class="mr-2" />
                    <label for="enable2FA" class="font-medium">Enable Two-Factor Authentication (2FA) for all users</label>
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
                        <Password id="paymentApiKey" v-model="settings.integrations.paymentGatewayApiKey" :feedback="false" toggleMask fluid />
                        <label for="paymentApiKey" class="font-medium mb-2">Payment Gateway API Key</label>
                    </FloatLabel>
                    <small class="text-gray-500 mt-1">Enter the API key for your payment processor (e.g., Stripe, PayPal).</small>
                  </div>
                </div>
              </div>
            </TabPanel>
          </TabPanels>
        </Tabs>

        <Toolbar class="mt-4">
            <template #end>
                <Button label="Save Settings" icon="pi pi-check" @click="saveSettings" />
            </template>
        </Toolbar>
      </template>
    </Card>
  </div>
</template>
