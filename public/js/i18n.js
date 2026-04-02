// Internationalization (i18n) module

const translations = {
    'zh-CN': {
        pageTitle: 'Sublink Worker - 订阅转换工具',
        pageDescription: '轻量级订阅转换和管理工具',
        navbarTitle: 'Sublink Worker',
        navbarGithub: 'GitHub',
        heroTitle: 'Sublink Worker',
        heroSubtitle: '轻量级订阅转换和管理工具',
        inputLabel: '输入订阅链接或配置',
        inputPlaceholder: '请输入订阅链接或 Base64 配置...',
        outputLabel: '输出格式',
        formatSingBox: 'Sing-Box',
        formatClash: 'Clash',
        formatSurge: 'Surge',
        formatXray: 'Xray',
        rulesLabel: '规则集',
        advancedOptions: '高级选项',
        customRulesLabel: '自定义规则 (JSON)',
        userAgentLabel: 'User-Agent',
        shortCodeLabel: '短链接代码',
        groupByCountry: '按国家分组',
        includeAutoSelect: '包含自动选择',
        generateBtn: '生成配置',
        createShortLink: '创建短链接',
        outputTitle: '输出结果',
        copyBtn: '复制链接',
        downloadBtn: '下载配置',
        outputUrlLabel: '订阅链接：',
        copyUrlBtn: '复制链接',
        shortlinkTitle: '短链接',
        footerText: '© 2024 Sublink Worker. MIT License.',
        github: 'GitHub',
        docs: '文档',
        generating: '生成中...',
        copySuccess: '已复制到剪贴板',
        copyError: '复制失败',
        downloadSuccess: '下载已开始',
        error: '发生错误',
        pleaseWait: '请稍候...'
    },
    'en': {
        pageTitle: 'Sublink Worker - Subscription Converter',
        pageDescription: 'Lightweight subscription converter and manager',
        navbarTitle: 'Sublink Worker',
        navbarGithub: 'GitHub',
        heroTitle: 'Sublink Worker',
        heroSubtitle: 'Lightweight subscription converter and manager',
        inputLabel: 'Input Subscription or Config',
        inputPlaceholder: 'Enter subscription URL or Base64 config...',
        outputLabel: 'Output Format',
        formatSingBox: 'Sing-Box',
        formatClash: 'Clash',
        formatSurge: 'Surge',
        formatXray: 'Xray',
        rulesLabel: 'Rule Sets',
        advancedOptions: 'Advanced Options',
        customRulesLabel: 'Custom Rules (JSON)',
        userAgentLabel: 'User-Agent',
        shortCodeLabel: 'Short Link Code',
        groupByCountry: 'Group by Country',
        includeAutoSelect: 'Include Auto Select',
        generateBtn: 'Generate Config',
        createShortLink: 'Create Short Link',
        outputTitle: 'Output',
        copyBtn: 'Copy Link',
        downloadBtn: 'Download Config',
        outputUrlLabel: 'Subscription URL:',
        copyUrlBtn: 'Copy Link',
        shortlinkTitle: 'Short Link',
        footerText: '© 2024 Sublink Worker. MIT License.',
        github: 'GitHub',
        docs: 'Docs',
        generating: 'Generating...',
        copySuccess: 'Copied to clipboard',
        copyError: 'Copy failed',
        downloadSuccess: 'Download started',
        error: 'An error occurred',
        pleaseWait: 'Please wait...'
    },
    'fa': {
        pageTitle: 'Sublink Worker - مبدل اشتراک',
        pageDescription: 'مبدل و مدیریت کننده سبک اشتراک',
        navbarTitle: 'Sublink Worker',
        navbarGithub: 'گیت‌هاب',
        heroTitle: 'Sublink Worker',
        heroSubtitle: 'مبدل و مدیریت کننده سبک اشتراک',
        inputLabel: 'ورودی اشتراک یا پیکربندی',
        inputPlaceholder: 'لینک اشتراک یا پیکربندی Base64 را وارد کنید...',
        outputLabel: 'فرمت خروجی',
        formatSingBox: 'Sing-Box',
        formatClash: 'Clash',
        formatSurge: 'Surge',
        formatXray: 'Xray',
        rulesLabel: 'مجموعه قوانین',
        advancedOptions: 'گزینه‌های پیشرفته',
        customRulesLabel: 'قوانین سفارشی (JSON)',
        userAgentLabel: 'User-Agent',
        shortCodeLabel: 'کد لینک کوتاه',
        groupByCountry: 'گروه بر اساس کشور',
        includeAutoSelect: 'شامل انتخاب خودکار',
        generateBtn: 'ایجاد پیکربندی',
        createShortLink: 'ایجاد لینک کوتاه',
        outputTitle: 'خروجی',
        copyBtn: 'کپی لینک',
        downloadBtn: 'دانلود پیکربندی',
        outputUrlLabel: 'لینک اشتراک:',
        copyUrlBtn: 'کپی لینک',
        shortlinkTitle: 'لینک کوتاه',
        footerText: '© 2024 Sublink Worker. مجوز MIT.',
        github: 'گیت‌هاب',
        docs: 'مستندات',
        generating: 'در حال ایجاد...',
        copySuccess: 'در کلیپ‌بورد کپی شد',
        copyError: 'کپی ناموفق بود',
        downloadSuccess: 'دانلود شروع شد',
        error: 'خطایی رخ داد',
        pleaseWait: 'لطفاً صبر کنید...'
    },
    'ru': {
        pageTitle: 'Sublink Worker - Конвертер подписок',
        pageDescription: 'Легкий конвертер и менеджер подписок',
        navbarTitle: 'Sublink Worker',
        navbarGithub: 'GitHub',
        heroTitle: 'Sublink Worker',
        heroSubtitle: 'Легкий конвертер и менеджер подписок',
        inputLabel: 'Ввод подписки или конфигурации',
        inputPlaceholder: 'Введите URL подписки или Base64 конфигурацию...',
        outputLabel: 'Формат вывода',
        formatSingBox: 'Sing-Box',
        formatClash: 'Clash',
        formatSurge: 'Surge',
        formatXray: 'Xray',
        rulesLabel: 'Наборы правил',
        advancedOptions: 'Дополнительные опции',
        customRulesLabel: 'Пользовательские правила (JSON)',
        userAgentLabel: 'User-Agent',
        shortCodeLabel: 'Код короткой ссылки',
        groupByCountry: 'Группировать по стране',
        includeAutoSelect: 'Включить автовыбор',
        generateBtn: 'Создать конфигурацию',
        createShortLink: 'Создать короткую ссылку',
        outputTitle: 'Вывод',
        copyBtn: 'Копировать ссылку',
        downloadBtn: 'Скачать конфигурацию',
        outputUrlLabel: 'URL подписки:',
        copyUrlBtn: 'Копировать ссылку',
        shortlinkTitle: 'Короткая ссылка',
        footerText: '© 2024 Sublink Worker. Лицензия MIT.',
        github: 'GitHub',
        docs: 'Документация',
        generating: 'Генерация...',
        copySuccess: 'Скопировано в буфер обмена',
        copyError: 'Ошибка копирования',
        downloadSuccess: 'Загрузка началась',
        error: 'Произошла ошибка',
        pleaseWait: 'Пожалуйста, подождите...'
    }
};

let currentLang = 'zh-CN';

function resolveLanguage() {
    // Check localStorage first
    const savedLang = localStorage.getItem('lang');
    if (savedLang && translations[savedLang]) {
        return savedLang;
    }

    // Try to get from navigator
    const browserLang = navigator.language || navigator.userLanguage;
    const lang = browserLang.split('-')[0];
    
    if (translations[lang]) {
        return lang;
    }
    if (translations[browserLang]) {
        return browserLang;
    }

    return 'zh-CN';
}

function setLanguage(lang) {
    if (!translations[lang]) {
        lang = 'zh-CN';
    }
    currentLang = lang;
    localStorage.setItem('lang', lang);
    document.documentElement.lang = lang;
    document.documentElement.dir = lang === 'fa' ? 'rtl' : 'ltr';
    updatePageText();
}

function updatePageText() {
    const t = translations[currentLang];
    
    // Update text content
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (t[key]) {
            el.textContent = t[key];
        }
    });

    // Update content attribute
    document.querySelectorAll('[data-i18n-content]').forEach(el => {
        const key = el.getAttribute('data-i18n-content');
        if (t[key]) {
            el.setAttribute('content', t[key]);
        }
    });

    // Update placeholder
    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
        const key = el.getAttribute('data-i18n-placeholder');
        if (t[key]) {
            el.setAttribute('placeholder', t[key]);
        }
    });

    // Update title
    document.querySelectorAll('[data-i18n-title]').forEach(el => {
        const key = el.getAttribute('data-i18n-title');
        if (t[key]) {
            el.setAttribute('title', t[key]);
        }
    });
}

function t(key) {
    return translations[currentLang][key] || translations['zh-CN'][key] || key;
}

// Initialize language on page load
document.addEventListener('DOMContentLoaded', () => {
    const lang = resolveLanguage();
    setLanguage(lang);
    
    // Set language selector value
    const selector = document.getElementById('languageSelector');
    if (selector) {
        selector.value = lang;
        selector.addEventListener('change', (e) => {
            setLanguage(e.target.value);
        });
    }
});

// Export for use in other scripts
window.i18n = {
    t,
    setLanguage,
    currentLang
};
