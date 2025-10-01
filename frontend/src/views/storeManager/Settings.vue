<script setup>
import { ref } from 'vue';
import Card from 'primevue/card';
import ToggleSwitch from 'primevue/toggleswitch';
import Select from 'primevue/select';
import Button from 'primevue/button';
import { useToast } from 'primevue/usetoast';
import { useThemeStore } from '../../store/theme';
import { storeToRefs } from 'pinia';

const themeStore = useThemeStore();
const { isDark } = storeToRefs(themeStore);

const toast = useToast();

const settings = ref({
    enableEmailNotifications: true,
    enableSmsNotifications: false,
    defaultCurrency: 'USD',
    timezone: 'America/New_York',
});

const timezones = ref([
    { label: 'Eastern Time (US & Canada)', value: 'America/New_York' },
    { label: 'Pacific Time (US & Canada)', value: 'America/Los_Angeles' },
    { label: 'Central Time (US & Canada)', value: 'America/Chicago' },
    { label: 'GMT/UTC', value: 'UTC' },
]);

const currencies = ref([
    { label: 'USD - United States Dollar', value: 'USD' },
    { label: 'EUR - Euro', value: 'EUR' },
    { label: 'JPY - Japanese Yen', value: 'JPY' },
]);

const saveSettings = () => {
    // In a real app, you would send this to the backend
    console.log('Saving settings:', settings.value);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Settings saved successfully', life: 3000 });
};

</script>

<template>
    <div>
        <Card>
            <template #title>
                <span class="text-2xl font-semibold">Store Settings</span>
            </template>
            <template #content>
                <div class="p-fluid formgrid grid">
                    <div class="field col-12">
                        <h3 class="text-lg font-semibold mb-4 border-b pb-2">Notifications</h3>
                        <div class="flex items-center mb-4">
                            <ToggleSwitch v-model="settings.enableEmailNotifications" inputId="email-notifications" class="mr-2" />
                            <label for="email-notifications">Enable Email Notifications</label>
                        </div>
                        <div class="flex items-center">
                            <ToggleSwitch v-model="settings.enableSmsNotifications" inputId="sms-notifications" class="mr-2" />
                            <label for="sms-notifications">Enable SMS Notifications</label>
                        </div>
                    </div>

                    <div class="field col-12">
                        <h3 class="text-lg font-semibold my-4 border-b pb-2">Appearance</h3>
                        <div class="flex items-center">
                            <ToggleSwitch v-model="isDark" inputId="theme-switcher" class="mr-2" />
                            <label for="theme-switcher">Dark Mode</label>
                        </div>
                    </div>

                    <div class="field col-12">
                        <h3 class="text-lg font-semibold my-4 border-b pb-2">Localization</h3>
                        <div class="field">
                            <label for="timezone">Timezone</label>
                            <Select id="timezone" v-model="settings.timezone" :options="timezones" optionLabel="label" optionValue="value" placeholder="Select a Timezone"></Select>
                        </div>
                        <div class="field mt-4">
                            <label for="currency">Default Currency</label>
                            <Select id="currency" v-model="settings.defaultCurrency" :options="currencies" optionLabel="label" optionValue="value" placeholder="Select a Currency"></Select>
                        </div>
                    </div>
                </div>
            </template>
            <template #footer>
                <div class="flex justify-end">
                    <Button label="Save Settings" icon="pi pi-check" severity="success" @click="saveSettings"></Button>
                </div>
            </template>
        </Card>
    </div>
</template>