// ===================================
// Arxis Landing Page - JavaScript
// The Mathematical Citadel
// ===================================

// ===================================
// Theme Toggle (Dark/Light Mode)
// ===================================
const themeToggle = document.getElementById('themeToggle');
const body = document.body;
const themeIcon = document.querySelector('.theme-icon');

// Load saved theme from localStorage
const currentTheme = localStorage.getItem('arxis-theme') || 'light';
body.setAttribute('data-theme', currentTheme);
themeIcon.textContent = currentTheme === 'dark' ? '☀️' : '🌙';

themeToggle.addEventListener('click', () => {
    const theme = body.getAttribute('data-theme') === 'light' ? 'dark' : 'light';
    body.setAttribute('data-theme', theme);
    localStorage.setItem('arxis-theme', theme);
    themeIcon.textContent = theme === 'dark' ? '☀️' : '🌙';
});

// ===================================
// Language Toggle (PT-BR/EN)
// ===================================
const langToggle = document.getElementById('langToggle');
const langCode = document.querySelector('.lang-code');
const flagIcon = document.querySelector('.flag');

const translations = {
    'pt-BR': {
        'nav-about': 'O que fazemos',
        'nav-how': 'Como funciona',
        'nav-impact': 'Por que importa',
        'nav-contact': 'Contato',
        'hero-badge': 'Ouvindo as Ondas do Espaço-Tempo',
        'hero-title': 'Detectando Colisões de Buracos Negros',
        'hero-subtitle': 'Quando dois buracos negros colidem, eles criam ondas no próprio tecido do espaço. Nós ajudamos cientistas a detectar e estudar esses eventos cósmicos.',
        'hero-description': 'É como jogar uma pedra num lago - as ondas se espalham. Só que no universo, são eventos milhões de vezes mais massivos que o Sol criando essas ondas.',
        'stat-1': 'Usado pela NASA/LISA',
        'stat-2': 'Detecta eventos a bilhões de anos-luz',
        'stat-3': 'Ciência real, impacto real',
        'cta-primary': 'Entenda como funciona',
        'cta-secondary': 'Por que isso importa',
    },
    'en': {
        'nav-about': 'What we do',
        'nav-how': 'How it works',
        'nav-impact': 'Why it matters',
        'nav-contact': 'Contact',
        'hero-badge': 'Listening to Spacetime Waves',
        'hero-title': 'Detecting Black Hole Collisions',
        'hero-subtitle': 'When two black holes collide, they create waves in the very fabric of space. We help scientists detect and study these cosmic events.',
        'hero-description': "It's like throwing a stone in a lake - the waves spread out. Except in the universe, these are events millions of times more massive than the Sun creating these waves.",
        'stat-1': 'Used by NASA/LISA',
        'stat-2': 'Detects events billions of light-years away',
        'stat-3': 'Real science, real impact',
        'cta-primary': 'Understand how it works',
        'cta-secondary': 'Why this matters',
    }
};

// Load saved language from localStorage (default: PT-BR)
let currentLang = localStorage.getItem('arxis-lang') || 'pt-BR';
langCode.textContent = currentLang === 'pt-BR' ? 'PT' : 'EN';
flagIcon.textContent = currentLang === 'pt-BR' ? '🇧🇷' : '🇬🇧';

function applyTranslations(lang) {
    const elements = document.querySelectorAll('[data-i18n]');
    elements.forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (translations[lang] && translations[lang][key]) {
            // Preserve HTML for spans like gradient-text
            if (el.querySelector('.gradient-text')) {
                const gradientText = el.querySelector('.gradient-text').textContent;
                if (lang === 'pt-BR') {
                    el.innerHTML = 'A <span class="gradient-text">Cidadela</span> Matemática';
                } else {
                    el.innerHTML = 'The Mathematical <span class="gradient-text">Citadel</span>';
                }
            } else {
                el.textContent = translations[lang][key];
            }
        }
    });
}

// Apply translations on load
applyTranslations(currentLang);

langToggle.addEventListener('click', () => {
    currentLang = currentLang === 'pt-BR' ? 'en' : 'pt-BR';
    localStorage.setItem('arxis-lang', currentLang);
    langCode.textContent = currentLang === 'pt-BR' ? 'PT' : 'EN';
    flagIcon.textContent = currentLang === 'pt-BR' ? '🇧🇷' : '🇬🇧';
    applyTranslations(currentLang);
});

document.addEventListener('DOMContentLoaded', function () {
    // ===================================
    // Smooth Scroll for Navigation Links
    // ===================================
    const navLinks = document.querySelectorAll('a[href^="#"]');

    navLinks.forEach(link => {
        link.addEventListener('click', function (e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            const targetSection = document.querySelector(targetId);

            if (targetSection) {
                const navHeight = document.querySelector('.navbar').offsetHeight;
                const targetPosition = targetSection.offsetTop - navHeight - 20;

                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });

    // ===================================
    // Navbar Scroll Effect
    // ===================================
    const navbar = document.querySelector('.navbar');
    let lastScroll = 0;

    window.addEventListener('scroll', () => {
        const currentScroll = window.pageYOffset;

        if (currentScroll > 100) {
            navbar.style.boxShadow = '0 4px 20px rgba(0, 212, 255, 0.15)';
        } else {
            navbar.style.boxShadow = 'none';
        }

        lastScroll = currentScroll;
    });

    // ===================================
    // Code Example Tabs
    // ===================================
    const tabButtons = document.querySelectorAll('.tab-button');
    const exampleContents = document.querySelectorAll('.example-content');

    tabButtons.forEach((button, index) => {
        button.addEventListener('click', () => {
            // Remove active class from all buttons and contents
            tabButtons.forEach(btn => btn.classList.remove('active'));
            exampleContents.forEach(content => content.classList.remove('active'));

            // Add active class to clicked button and corresponding content
            button.classList.add('active');
            const tabName = button.getAttribute('data-tab');
            const content = document.getElementById(`example-${tabName}`);
            if (content) {
                content.classList.add('active');
            }
        });
    });

    // ===================================
    // Copy Installation Code
    // ===================================
    window.copyInstallCode = function () {
        const code = `[dependencies]
arxis_quaternions = "0.2.0"
avila-math = { git = "https://github.com/avilaops/arxis" }
avila-telemetry = { git = "https://github.com/avilaops/arxis" }`;

        // Create temporary textarea
        const textarea = document.createElement('textarea');
        textarea.value = code;
        textarea.style.position = 'fixed';
        textarea.style.opacity = '0';
        document.body.appendChild(textarea);

        // Select and copy
        textarea.select();
        document.execCommand('copy');

        // Remove textarea
        document.body.removeChild(textarea);

        // Visual feedback
        const button = document.querySelector('.copy-button');
        const originalHTML = button.innerHTML;

        button.innerHTML = `
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
        `;

        button.style.background = 'rgba(0, 200, 83, 0.3)';
        button.style.borderColor = '#00C853';
        button.style.color = '#00C853';

        setTimeout(() => {
            button.innerHTML = originalHTML;
            button.style.background = 'rgba(0, 212, 255, 0.2)';
            button.style.borderColor = '#00D4FF';
            button.style.color = '#00D4FF';
        }, 2000);
    };

    // ===================================
    // Intersection Observer for Animations
    // ===================================
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, observerOptions);

    // Observe feature cards
    const featureCards = document.querySelectorAll('.feature-card');
    featureCards.forEach((card, index) => {
        card.style.opacity = '0';
        card.style.transform = 'translateY(30px)';
        card.style.transition = `all 0.6s ease ${index * 0.1}s`;
        observer.observe(card);
    });

    // Observe doc cards
    const docCards = document.querySelectorAll('.doc-card');
    docCards.forEach((card, index) => {
        card.style.opacity = '0';
        card.style.transform = 'translateY(30px)';
        card.style.transition = `all 0.6s ease ${index * 0.1}s`;
        observer.observe(card);
    });

    // Observe stat cards
    const statCards = document.querySelectorAll('.stat-card');
    statCards.forEach((card, index) => {
        card.style.opacity = '0';
        card.style.transform = 'translateY(30px)';
        card.style.transition = `all 0.6s ease ${index * 0.1}s`;
        observer.observe(card);
    });

    // ===================================
    // Counter Animation for Stats
    // ===================================
    function animateCounter(element, target, duration = 2000) {
        let start = 0;
        const increment = target / (duration / 16);
        const isNumber = !isNaN(target);

        const timer = setInterval(() => {
            start += increment;
            if (start >= target) {
                clearInterval(timer);
                start = target;
            }

            if (isNumber) {
                element.textContent = Math.floor(start);
            }
        }, 16);
    }

    // Animate stats when they come into view
    const statNumbers = document.querySelectorAll('.stat-number');
    const statsObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const target = entry.target.textContent;
                const numericValue = parseInt(target);

                if (!isNaN(numericValue)) {
                    entry.target.textContent = '0';
                    animateCounter(entry.target, numericValue);
                }

                statsObserver.unobserve(entry.target);
            }
        });
    }, observerOptions);

    statNumbers.forEach(stat => {
        if (!isNaN(parseInt(stat.textContent))) {
            statsObserver.observe(stat);
        }
    });

    // ===================================
    // Parallax Effect for Hero Background
    // ===================================
    const heroBackground = document.querySelector('.gravitational-wave');

    if (heroBackground) {
        window.addEventListener('scroll', () => {
            const scrolled = window.pageYOffset;
            const parallax = scrolled * 0.5;
            heroBackground.style.transform = `translate(-25%, -25%) scale(1.2) rotate(${parallax * 0.1}deg)`;
        });
    }

    // ===================================
    // Dynamic Star Generation
    // ===================================
    const starsContainer = document.querySelector('.stars');

    if (starsContainer) {
        // Add more dynamic stars
        for (let i = 0; i < 50; i++) {
            const star = document.createElement('div');
            star.className = 'dynamic-star';
            star.style.cssText = `
                position: absolute;
                width: ${Math.random() * 3 + 1}px;
                height: ${Math.random() * 3 + 1}px;
                background: white;
                border-radius: 50%;
                top: ${Math.random() * 100}%;
                left: ${Math.random() * 100}%;
                animation: twinkle ${Math.random() * 3 + 2}s ease-in-out infinite;
                opacity: ${Math.random() * 0.5 + 0.3};
            `;
            starsContainer.appendChild(star);
        }
    }

    // ===================================
    // Keyboard Navigation
    // ===================================
    document.addEventListener('keydown', (e) => {
        // Press 'g' + 'h' to go to GitHub
        if (e.key === 'g') {
            const nextKey = (ne) => {
                if (ne.key === 'h') {
                    window.open('https://github.com/avilaops/arxis', '_blank');
                }
                document.removeEventListener('keydown', nextKey);
            };
            document.addEventListener('keydown', nextKey);
            setTimeout(() => document.removeEventListener('keydown', nextKey), 1000);
        }
    });

    // ===================================
    // Mobile Menu Toggle (if needed)
    // ===================================
    const createMobileMenu = () => {
        if (window.innerWidth <= 768) {
            const navLinks = document.querySelector('.nav-links');
            if (navLinks && !document.querySelector('.mobile-menu-toggle')) {
                // Create hamburger menu
                const menuToggle = document.createElement('button');
                menuToggle.className = 'mobile-menu-toggle';
                menuToggle.innerHTML = `
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="3" y1="12" x2="21" y2="12"></line>
                        <line x1="3" y1="6" x2="21" y2="6"></line>
                        <line x1="3" y1="18" x2="21" y2="18"></line>
                    </svg>
                `;

                menuToggle.style.cssText = `
                    background: none;
                    border: none;
                    color: var(--color-cyan);
                    cursor: pointer;
                    padding: 0.5rem;
                `;

                menuToggle.addEventListener('click', () => {
                    navLinks.style.display = navLinks.style.display === 'flex' ? 'none' : 'flex';
                    if (navLinks.style.display === 'flex') {
                        navLinks.style.cssText = `
                            position: absolute;
                            top: 100%;
                            left: 0;
                            right: 0;
                            background: rgba(10, 22, 40, 0.98);
                            flex-direction: column;
                            padding: 1rem;
                            border-top: 1px solid rgba(0, 212, 255, 0.2);
                        `;
                    }
                });

                const navContainer = document.querySelector('.nav-container');
                navContainer.appendChild(menuToggle);
            }
        }
    };

    createMobileMenu();
    window.addEventListener('resize', createMobileMenu);

    // ===================================
    // Easter Egg: Konami Code
    // ===================================
    const konamiCode = ['ArrowUp', 'ArrowUp', 'ArrowDown', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'ArrowLeft', 'ArrowRight', 'b', 'a'];
    let konamiIndex = 0;

    document.addEventListener('keydown', (e) => {
        if (e.key === konamiCode[konamiIndex]) {
            konamiIndex++;
            if (konamiIndex === konamiCode.length) {
                // Easter egg activated!
                document.body.style.animation = 'rainbow 5s linear infinite';

                const style = document.createElement('style');
                style.textContent = `
                    @keyframes rainbow {
                        0% { filter: hue-rotate(0deg); }
                        100% { filter: hue-rotate(360deg); }
                    }
                `;
                document.head.appendChild(style);

                setTimeout(() => {
                    document.body.style.animation = '';
                    style.remove();
                }, 5000);

                konamiIndex = 0;
            }
        } else {
            konamiIndex = 0;
        }
    });

    // ===================================
    // Performance Monitoring
    // ===================================
    if ('PerformanceObserver' in window) {
        const perfObserver = new PerformanceObserver((list) => {
            for (const entry of list.getEntries()) {
                if (entry.entryType === 'largest-contentful-paint') {
                    console.log('LCP:', entry.renderTime || entry.loadTime);
                }
            }
        });

        try {
            perfObserver.observe({ entryTypes: ['largest-contentful-paint'] });
        } catch (e) {
            // Browser doesn't support this metric
        }
    }

    // ===================================
    // Console Easter Egg
    // ===================================
    console.log('%c🏛️ Arxis - The Mathematical Citadel', 'font-size: 20px; font-weight: bold; color: #00D4FF;');
    console.log('%cResearch-Grade Physics & Mathematics in Rust', 'font-size: 14px; color: #8B4FE8;');
    console.log('%cGitHub: https://github.com/avilaops/arxis', 'font-size: 12px; color: #E8F1F5;');
    console.log('%cInterested in contributing? Check out our repo!', 'font-size: 12px; color: #FFB800;');

    console.log('\n%cKnow the Konami Code? Try it! ⬆⬆⬇⬇⬅➡⬅➡BA', 'font-size: 10px; color: #676E95;');

    // ===================================
    // Analytics (placeholder)
    // ===================================
    function trackEvent(category, action, label) {
        // Placeholder for analytics
        // Implement with Google Analytics, Plausible, or other
        console.log('Event:', category, action, label);
    }

    // Track button clicks
    document.querySelectorAll('.btn').forEach(btn => {
        btn.addEventListener('click', () => {
            trackEvent('Button', 'Click', btn.textContent.trim());
        });
    });

    // Track external links
    document.querySelectorAll('a[target="_blank"]').forEach(link => {
        link.addEventListener('click', () => {
            trackEvent('External Link', 'Click', link.href);
        });
    });

    // ===================================
    // Initialization Complete
    // ===================================
    console.log('%c✅ Arxis landing page initialized', 'color: #00C853; font-weight: bold;');
});

// ===================================
// Service Worker Registration (Progressive Web App)
// ===================================
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        // Uncomment when you have a service worker file
        // navigator.serviceWorker.register('/sw.js')
        //     .then(reg => console.log('Service Worker registered'))
        //     .catch(err => console.log('Service Worker registration failed'));
    });
}
