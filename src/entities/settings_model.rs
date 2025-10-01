use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub general: GeneralSettings,
    pub receipt: ReceiptSettings,
    pub security: SecuritySettings,
    pub integrations: IntegrationsSettings,
    pub email: EmailSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralSettings {
    #[serde(rename = "siteName")]
    pub site_name: String,
    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    #[serde(rename = "defaultTaxRate")]
    pub default_tax_rate: f64,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "defaultDateFormat")]
    pub default_date_format: String,
    #[serde(rename = "enablePromotions")]
    pub enable_promotions: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReceiptSettings {
    #[serde(rename = "headerText")]
    pub header_text: String,
    #[serde(rename = "footerText")]
    pub footer_text: String,
    #[serde(rename = "showStoreAddress")]
    pub show_store_address: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecuritySettings {
    #[serde(rename = "sessionTimeout")]
    pub session_timeout: i32,
    #[serde(rename = "enable2FA")]
    pub enable_2fa: bool,
    #[serde(rename = "passwordPolicy")]
    pub password_policy: PasswordPolicy,
    #[serde(rename = "accountLockout")]
    pub account_lockout: AccountLockout,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordPolicy {
    #[serde(rename = "minLength")]
    pub min_length: i32,
    #[serde(rename = "requireUppercase")]
    pub require_uppercase: bool,
    #[serde(rename = "requireLowercase")]
    pub require_lowercase: bool,
    #[serde(rename = "requireNumbers")]
    pub require_numbers: bool,
    #[serde(rename = "requireSymbols")]
    pub require_symbols: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountLockout {
    #[serde(rename = "maxFailedAttempts")]
    pub max_failed_attempts: i32,
    #[serde(rename = "lockoutDuration")]
    pub lockout_duration: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationsSettings {
    #[serde(rename = "paymentGatewayApiKey")]
    pub payment_gateway_api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailSettings {
    #[serde(rename = "fromName")]
    pub from_name: String,
    #[serde(rename = "fromEmail")]
    pub from_email: String,
    #[serde(rename = "smtpServer")]
    pub smtp_server: String,
    #[serde(rename = "smtpPort")]
    pub smtp_port: i32,
    #[serde(rename = "smtpUsername")]
    pub smtp_username: String,
    #[serde(rename = "smtpPassword")]
    pub smtp_password: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                site_name: "My POS".to_string(),
                logo_url: "".to_string(),
                default_tax_rate: 0.0,
                currency_symbol: "$".to_string(),
                currency_code: "USD".to_string(),
                default_date_format: "MM/DD/YYYY".to_string(),
                enable_promotions: true,
            },
            receipt: ReceiptSettings {
                header_text: "Thank you for your purchase!".to_string(),
                footer_text: "Please come again.".to_string(),
                show_store_address: true,
            },
            security: SecuritySettings {
                session_timeout: 30,
                enable_2fa: false,
                password_policy: PasswordPolicy {
                    min_length: 8,
                    require_uppercase: true,
                    require_lowercase: true,
                    require_numbers: true,
                    require_symbols: false,
                },
                account_lockout: AccountLockout {
                    max_failed_attempts: 5,
                    lockout_duration: 15,
                },
            },
            integrations: IntegrationsSettings {
                payment_gateway_api_key: "".to_string(),
            },
            email: EmailSettings {
                from_name: "My POS".to_string(),
                from_email: "noreply@mypos.com".to_string(),
                smtp_server: "".to_string(),
                smtp_port: 587,
                smtp_username: "".to_string(),
                smtp_password: "".to_string(),
            },
        }
    }
}
