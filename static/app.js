// ==== Тэма ====
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

// ==== Эфэкт «сьвятла» на картках ====
document.querySelectorAll('.card').forEach(card => {
    card.addEventListener('mousemove', e => {
        const r = card.getBoundingClientRect();
        card.style.setProperty('--mx', `${e.clientX - r.left}px`);
        card.style.setProperty('--my', `${e.clientY - r.top}px`);
    });
});

// ==== Паяўленьне пры скроле ====
const io = new IntersectionObserver(entries => {
    entries.forEach(en => {
        if (en.isIntersecting) {
            en.target.style.animationPlayState = 'running';
            io.unobserve(en.target);
        }
    });
}, { threshold: 0.1 });

document.querySelectorAll('.card, .panel, .timeline li, .word').forEach(el => {
    el.style.animation = 'fadeUp 0.7s ease both';
    el.style.animationPlayState = 'paused';
    io.observe(el);
});