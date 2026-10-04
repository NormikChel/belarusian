const root = document.documentElement;
const btn = document.getElementById('themeToggle');
const saved = localStorage.getItem('mova-theme');
if (saved) root.setAttribute('data-theme', saved);
else if (window.matchMedia('(prefers-color-scheme: light)').matches)
    root.setAttribute('data-theme', 'light');
btn?.addEventListener('click', () => {
    const cur = root.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
    root.setAttribute('data-theme', cur);
    localStorage.setItem('mova-theme', cur);
});