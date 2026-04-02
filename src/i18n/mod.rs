//! Internationalization (i18n) module

use fluent_bundle::{FluentBundle, FluentResource};
use fluent::FluentArgs;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

/// Translator for i18n
pub struct Translator {
    bundle: FluentBundle<FluentResource>,
}

impl Translator {
    pub fn new(lang: &str) -> Self {
        let lang_id: LanguageIdentifier = lang.parse().unwrap_or_else(|_| "zh-CN".parse().unwrap());
        let mut bundle = FluentBundle::new(vec![lang_id]);
        
        // Add default translations
        Self::add_default_resources(&mut bundle, lang);
        
        Translator { bundle }
    }
    
    fn add_default_resources(bundle: &mut FluentBundle<FluentResource>, lang: &str) {
        let resources = match lang {
            "en" | "en-US" => Self::get_english_resources(),
            "fa" | "fa-IR" => Self::get_persian_resources(),
            "ru" | "ru-RU" => Self::get_russian_resources(),
            _ => Self::get_chinese_resources(), // Default to Chinese
        };
        
        for res in resources {
            bundle.add_resource(res).ok();
        }
    }
    
    fn get_chinese_resources() -> Vec<FluentResource> {
        vec![
            FluentResource::try_new(
                r#"
outboundName-AutoSelect = ⚡ 自动选择
outboundName-NodeSelect = 🚀 节点选择
outboundName-FallBack = 🐟 漏网之鱼
outboundName-ManualSwitch = 🖐️ 手动切换
outboundName-AdBlock = 🛑 广告拦截
outboundName-AIServices = 💬 AI 服务
outboundName-Bilibili = 📺 哔哩哔哩
outboundName-Youtube = 📹 油管视频
outboundName-Google = 🔍 谷歌服务
outboundName-Private = 🏠 私有网络
outboundName-LocationCN = 🔒 国内服务
outboundName-Telegram = 📲 电报消息
outboundName-Github = 🐱 Github
outboundName-Microsoft = Ⓜ️ 微软服务
outboundName-Apple = 🍏 苹果服务
outboundName-SocialMedia = 🌐 社交媒体
outboundName-Streaming = 🎬 流媒体
outboundName-Gaming = 🎮 游戏平台
outboundName-Education = 📚 教育资源
outboundName-Financial = 💰 金融服务
outboundName-CloudServices = ☁️ 云服务
outboundName-Non-China = 🌐 非中国
"#.to_string()
            ).unwrap(),
        ]
    }
    
    fn get_english_resources() -> Vec<FluentResource> {
        vec![
            FluentResource::try_new(
                r#"
outboundName-AutoSelect = ⚡ Auto Select
outboundName-NodeSelect = 🚀 Node Select
outboundName-FallBack = 🐟 Fall Back
outboundName-ManualSwitch = 🖐️ Manual Switch
outboundName-AdBlock = 🛑 Ad Blocking
outboundName-AIServices = 💬 AI Services
outboundName-Bilibili = 📺 Bilibili
outboundName-Youtube = 📹 Youtube
outboundName-Google = 🔍 Google Services
outboundName-Private = 🏠 Private Network
outboundName-LocationCN = 🔒 China Services
outboundName-Telegram = 📲 Telegram
outboundName-Github = 🐱 Github
outboundName-Microsoft = Ⓜ️ Microsoft Services
outboundName-Apple = 🍏 Apple Services
outboundName-SocialMedia = 🌐 Social Media
outboundName-Streaming = 🎬 Streaming
outboundName-Gaming = 🎮 Gaming Platform
outboundName-Education = 📚 Education Resources
outboundName-Financial = 💰 Financial Services
outboundName-CloudServices = ☁️ Cloud Services
outboundName-Non-China = 🌐 Non-China
"#.to_string()
            ).unwrap(),
        ]
    }
    
    fn get_persian_resources() -> Vec<FluentResource> {
        vec![
            FluentResource::try_new(
                r#"
outboundName-AutoSelect = ⚡ انتخاب خودکار
outboundName-NodeSelect = 🚀 انتخاب نود
outboundName-FallBack = 🐟 فال بک
outboundName-ManualSwitch = 🖐️ انتخاب دستی
outboundName-AdBlock = 🛑 مسدودسازی تبلیغات
outboundName-AIServices = 💬 سرویس‌های هوش مصنوعی
outboundName-Bilibili = 📺 بیلی‌بیلی
outboundName-Youtube = 📹 یوتیوب
outboundName-Google = 🔍 سرویس‌های گوگل
outboundName-Private = 🏠 شبکه خصوصی
outboundName-LocationCN = 🔒 سرویس‌های چین
outboundName-Telegram = 📲 تلگرام
outboundName-Github = 🐱 گیت‌هاب
outboundName-Microsoft = Ⓜ️ سرویس‌های مایکروسافت
outboundName-Apple = 🍏 سرویس‌های اپل
outboundName-SocialMedia = 🌐 شبکه‌های اجتماعی
outboundName-Streaming = 🎬 استریمینگ
outboundName-Gaming = 🎮 پلتفرم بازی
outboundName-Education = 📚 منابع آموزشی
outboundName-Financial = 💰 سرویس‌های مالی
outboundName-CloudServices = ☁️ سرویس‌های ابری
outboundName-Non-China = 🌐 خارج از چین
"#.to_string()
            ).unwrap(),
        ]
    }
    
    fn get_russian_resources() -> Vec<FluentResource> {
        vec![
            FluentResource::try_new(
                r#"
outboundName-AutoSelect = ⚡ Автовыбор
outboundName-NodeSelect = 🚀 Выбор узла
outboundName-FallBack = 🐟 Резерв
outboundName-ManualSwitch = 🖐️ Ручной выбор
outboundName-AdBlock = 🛑 Блокировка рекламы
outboundName-AIServices = 💬 AI-сервисы
outboundName-Bilibili = 📺 Bilibili
outboundName-Youtube = 📹 YouTube
outboundName-Google = 🔍 Сервисы Google
outboundName-Private = 🏠 Локальная сеть
outboundName-LocationCN = 🔒 Сервисы Китая
outboundName-Telegram = 📲 Telegram
outboundName-Github = 🐱 GitHub
outboundName-Microsoft = Ⓜ️ Сервисы Microsoft
outboundName-Apple = 🍏 Сервисы Apple
outboundName-SocialMedia = 🌐 Социальные сети
outboundName-Streaming = 🎬 Стриминг
outboundName-Gaming = 🎮 Игровые платформы
outboundName-Education = 📚 Образовательные ресурсы
outboundName-Financial = 💰 Финансовые сервисы
outboundName-CloudServices = ☁️ Облачные сервисы
outboundName-Non-China = 🌐 За пределами Китая
"#.to_string()
            ).unwrap(),
        ]
    }
    
    pub fn get(&self, key: &str, default: &str) -> String {
        let mut errors = vec![];
        if let Some(message) = self.bundle.get_message(key) {
            if let Some(pattern) = message.value() {
                return self.bundle.format_pattern(pattern, None, &mut errors).to_string();
            }
        }
        default.to_string()
    }
}

/// Create a translator for the given language
pub fn create_translator(lang: &str) -> Translator {
    Translator::new(lang)
}

/// Resolve language from accept-language header or default
pub fn resolve_language(accept_language: Option<&str>) -> String {
    if let Some(header) = accept_language {
        // Parse accept-language header
        if let Some(primary) = header.split(',').next() {
            return primary.trim().to_string();
        }
    }
    "zh-CN".to_string()
}
