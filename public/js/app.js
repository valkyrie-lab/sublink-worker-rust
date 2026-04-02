// Main application JavaScript

document.addEventListener('DOMContentLoaded', () => {
    // DOM Elements
    const converterForm = document.getElementById('converterForm');
    const inputText = document.getElementById('inputText');
    const outputCard = document.getElementById('outputCard');
    const outputContent = document.getElementById('outputContent');
    const outputUrl = document.getElementById('outputUrl');
    const shortlinkCard = document.getElementById('shortlinkCard');
    const shortlinkUrl = document.getElementById('shortlinkUrl');
    const generateBtn = document.getElementById('generateBtn');
    const createShortLinkBtn = document.getElementById('createShortLinkBtn');
    const copyBtn = document.getElementById('copyBtn');
    const downloadBtn = document.getElementById('downloadBtn');
    const copyUrlBtn = document.getElementById('copyUrlBtn');
    const copyShortlinkBtn = document.getElementById('copyShortlinkBtn');
    const advancedToggle = document.getElementById('advancedToggle');
    const advancedOptions = document.getElementById('advancedOptions');
    const themeToggle = document.getElementById('themeToggle');
    const toast = document.getElementById('toast');

    let currentOutputUrl = '';
    let currentShortlinkUrl = '';

    // Theme toggle
    function initTheme() {
        const savedTheme = localStorage.getItem('theme');
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const theme = savedTheme || (prefersDark ? 'dark' : 'light');
        setTheme(theme);
    }

    function setTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);
        localStorage.setItem('theme', theme);
        const icon = themeToggle.querySelector('.theme-icon');
        icon.textContent = theme === 'dark' ? '☀️' : '🌙';
    }

    themeToggle.addEventListener('click', () => {
        const currentTheme = document.documentElement.getAttribute('data-theme');
        const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
        setTheme(newTheme);
    });

    // Advanced options toggle
    advancedToggle.addEventListener('click', () => {
        const isHidden = advancedOptions.classList.contains('hidden');
        advancedOptions.classList.toggle('hidden');
        advancedToggle.setAttribute('aria-expanded', isHidden);
    });

    // Show toast notification
    function showToast(message, type = 'success') {
        toast.textContent = message;
        toast.className = `toast ${type}`;
        toast.classList.remove('hidden');
        setTimeout(() => {
            toast.classList.add('hidden');
        }, 3000);
    }

    // Copy to clipboard
    async function copyToClipboard(text) {
        try {
            await navigator.clipboard.writeText(text);
            showToast(window.i18n.t('copySuccess'), 'success');
            return true;
        } catch (err) {
            // Fallback for older browsers
            const textarea = document.createElement('textarea');
            textarea.value = text;
            document.body.appendChild(textarea);
            textarea.select();
            document.execCommand('copy');
            document.body.removeChild(textarea);
            showToast(window.i18n.t('copySuccess'), 'success');
            return true;
        }
    }

    // Generate config URL
    function generateConfigUrl(format) {
        const input = inputText.value.trim();
        if (!input) {
            showToast('请输入订阅链接或配置', 'error');
            inputText.focus();
            return null;
        }

        const baseUrl = window.location.origin;
        const params = new URLSearchParams();
        params.set('config', input);

        // Get selected rules
        const selectedRules = Array.from(document.querySelectorAll('input[name="selectedRules"]:checked'))
            .map(cb => cb.value);
        if (selectedRules.length > 0) {
            params.set('selectedRules', JSON.stringify(selectedRules));
        }

        // Get advanced options
        const customRules = document.getElementById('customRules').value.trim();
        if (customRules) {
            params.set('customRules', customRules);
        }

        const userAgent = document.getElementById('userAgent').value.trim();
        if (userAgent && userAgent !== 'curl/7.74.0') {
            params.set('ua', userAgent);
        }

        const groupByCountry = document.querySelector('input[name="groupByCountry"]:checked');
        if (groupByCountry) {
            params.set('group_by_country', 'true');
        }

        const includeAutoSelect = document.querySelector('input[name="includeAutoSelect"]:checked');
        if (!includeAutoSelect) {
            params.set('include_auto_select', 'false');
        }

        // Get language
        params.set('lang', window.i18n.currentLang);

        return `${baseUrl}/${format}?${params.toString()}`;
    }

    // Generate config
    async function generateConfig(format) {
        const url = generateConfigUrl(format);
        if (!url) return;

        // Update button state
        generateBtn.classList.add('loading');
        generateBtn.querySelector('span:last-child').textContent = window.i18n.t('generating');
        generateBtn.disabled = true;

        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const contentType = response.headers.get('content-type');
            let data;

            if (contentType && contentType.includes('application/json')) {
                data = await response.json();
                data = JSON.stringify(data, null, 2);
            } else {
                data = await response.text();
            }

            // Display output
            outputContent.textContent = data;
            currentOutputUrl = url;
            outputUrl.value = url;
            outputCard.classList.remove('hidden');
            shortlinkCard.classList.add('hidden');

            showToast('配置生成成功', 'success');
        } catch (error) {
            console.error('Error:', error);
            showToast(`${window.i18n.t('error')}: ${error.message}`, 'error');
        } finally {
            // Reset button state
            generateBtn.classList.remove('loading');
            generateBtn.querySelector('span:last-child').textContent = window.i18n.t('generateBtn');
            generateBtn.disabled = false;
        }
    }

    // Create short link
    async function createShortLink() {
        const url = generateConfigUrl('singbox');
        if (!url) return;

        const shortCode = document.getElementById('shortCode').value.trim();
        const shortLinkUrl = shortCode 
            ? `${window.location.origin}/shorten-v2?url=${encodeURIComponent(url)}&shortCode=${encodeURIComponent(shortCode)}`
            : `${window.location.origin}/shorten-v2?url=${encodeURIComponent(url)}`;

        try {
            const response = await fetch(shortLinkUrl);
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const code = await response.text();
            currentShortlinkUrl = `${window.location.origin}/b/${code}`;
            shortlinkUrl.value = currentShortlinkUrl;
            shortlinkCard.classList.remove('hidden');
            
            showToast('短链接创建成功', 'success');
        } catch (error) {
            console.error('Error:', error);
            showToast(`${window.i18n.t('error')}: ${error.message}`, 'error');
        }
    }

    // Download config
    function downloadConfig() {
        const content = outputContent.textContent;
        const format = document.querySelector('input[name="outputFormat"]:checked').value;
        
        const extensions = {
            singbox: 'json',
            clash: 'yaml',
            surge: 'conf',
            xray: 'txt'
        };

        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `sublink-config.${extensions[format] || 'txt'}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);

        showToast(window.i18n.t('downloadSuccess'), 'success');
    }

    // Event listeners
    converterForm.addEventListener('submit', (e) => {
        e.preventDefault();
        const format = document.querySelector('input[name="outputFormat"]:checked').value;
        generateConfig(format);
    });

    createShortLinkBtn.addEventListener('click', createShortLink);

    copyBtn.addEventListener('click', () => {
        copyToClipboard(outputContent.textContent);
    });

    downloadBtn.addEventListener('click', downloadConfig);

    copyUrlBtn.addEventListener('click', () => {
        copyToClipboard(outputUrl.value);
    });

    copyShortlinkBtn.addEventListener('click', () => {
        copyToClipboard(shortlinkUrl.value);
    });

    // Initialize
    initTheme();
});
