// ===================================
// Arxis Landing Page - JavaScript
// Novo design compacto e didático
// ===================================

document.addEventListener('DOMContentLoaded', function () {

    // ===================================
    // Smooth Scroll
    // ===================================
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                const navHeight = document.querySelector('.navbar').offsetHeight;
                const targetPosition = target.offsetTop - navHeight - 20;
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
            navbar.classList.add('scrolled');
        } else {
            navbar.classList.remove('scrolled');
        }

        lastScroll = currentScroll;
    });

    // ===================================
    // Scroll Reveal Animation
    // ===================================
    const revealElements = document.querySelectorAll('.explanation-card, .use-case-card, .step-card, .user-card, .why-rust-card, .contact-card');

    const revealObserver = new IntersectionObserver((entries) => {
        entries.forEach((entry, index) => {
            if (entry.isIntersecting) {
                setTimeout(() => {
                    entry.target.classList.add('reveal', 'active');
                }, index * 100);
                revealObserver.unobserve(entry.target);
            }
        });
    }, {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    });

    revealElements.forEach(el => {
        el.classList.add('reveal');
        revealObserver.observe(el);
    });

    // ===================================
    // Counter Animation for Stats
    // ===================================
    const statNumbers = document.querySelectorAll('.stat-highlight-number');

    function animateCounter(element, target, duration = 2000) {
        const start = 0;
        const increment = target / (duration / 16);
        let current = start;

        const timer = setInterval(() => {
            current += increment;
            if (current >= target) {
                clearInterval(timer);
                current = target;
            }
            element.textContent = Math.floor(current);
        }, 16);
    }

    const statsObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const target = parseInt(entry.target.textContent);
                if (!isNaN(target)) {
                    entry.target.textContent = '0';
                    animateCounter(entry.target, target);
                }
                statsObserver.unobserve(entry.target);
            }
        });
    }, { threshold: 0.5 });

    statNumbers.forEach(stat => {
        statsObserver.observe(stat);
    });

    // ===================================
    // Code Snippet Copy Functionality
    // ===================================
    const codeSnippets = document.querySelectorAll('.code-snippet');

    codeSnippets.forEach(snippet => {
        // Add copy button
        const copyBtn = document.createElement('button');
        copyBtn.innerHTML = '📋';
        copyBtn.className = 'code-copy-btn';
        copyBtn.style.cssText = `
            position: absolute;
            top: 10px;
            right: 10px;
            background: rgba(255, 140, 0, 0.8);
            border: none;
            color: white;
            padding: 0.5rem;
            border-radius: 6px;
            cursor: pointer;
            font-size: 1rem;
            transition: all 0.3s ease;
        `;

        snippet.parentElement.style.position = 'relative';
        snippet.parentElement.appendChild(copyBtn);

        copyBtn.addEventListener('click', () => {
            const code = snippet.textContent;
            navigator.clipboard.writeText(code).then(() => {
                copyBtn.innerHTML = '✅';
                copyBtn.style.background = 'rgba(76, 175, 80, 0.8)';

                setTimeout(() => {
                    copyBtn.innerHTML = '📋';
                    copyBtn.style.background = 'rgba(255, 140, 0, 0.8)';
                }, 2000);
            });
        });
    });

    // ===================================
    // Table of Contents Highlight (optional)
    // ===================================
    const sections = document.querySelectorAll('section[id]');
    const navLinks = document.querySelectorAll('.nav-links a[href^="#"]');

    window.addEventListener('scroll', () => {
        let current = '';

        sections.forEach(section => {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.clientHeight;
            if (window.pageYOffset >= sectionTop - 100) {
                current = section.getAttribute('id');
            }
        });

        navLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === `#${current}`) {
                link.classList.add('active');
            }
        });
    });

    // ===================================
    // Mobile Menu Toggle
    // ===================================
    function createMobileMenu() {
        if (window.innerWidth <= 768) {
            const navLinks = document.querySelector('.nav-links');
            const navContainer = document.querySelector('.nav-container');

            if (!document.querySelector('.mobile-menu-toggle')) {
                const menuToggle = document.createElement('button');
                menuToggle.className = 'mobile-menu-toggle';
                menuToggle.innerHTML = '☰';
                menuToggle.style.cssText = `
                    display: block;
                    background: var(--gradient-primary);
                    border: none;
                    color: white;
                    font-size: 1.5rem;
                    padding: 0.5rem 1rem;
                    border-radius: 8px;
                    cursor: pointer;
                `;

                menuToggle.addEventListener('click', () => {
                    navLinks.style.display = navLinks.style.display === 'flex' ? 'none' : 'flex';
                    if (navLinks.style.display === 'flex') {
                        navLinks.style.cssText = `
                            position: absolute;
                            top: 100%;
                            left: 0;
                            right: 0;
                            background: white;
                            flex-direction: column;
                            padding: 1rem;
                            box-shadow: 0 4px 16px rgba(0,0,0,0.1);
                            z-index: 1000;
                        `;
                    }
                });

                navContainer.appendChild(menuToggle);
            }
        }
    }

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
    console.log('%c🏛️ Arxis - A Cidadela Matemática', 'font-size: 24px; font-weight: bold; background: linear-gradient(135deg, #FFD700, #FF8C00); -webkit-background-clip: text; -webkit-text-fill-color: transparent;');
    console.log('%c🚀 Biblioteca Científica Brasileira', 'font-size: 16px; color: #FF8C00;');
    console.log('%c✨ Made with ❤️ by Nicolas Ávila', 'font-size: 12px; color: #666;');
    console.log('%c📚 GitHub: https://github.com/avilaops/arxis', 'font-size: 12px; color: #4A90E2;');
    console.log('\n%c🎮 Dica: Tente o código Konami! ⬆⬆⬇⬇⬅➡⬅➡BA', 'font-size: 10px; color: #999;');

    // ===================================
    // Analytics Placeholder
    // ===================================
    function trackEvent(category, action, label) {
        // Placeholder for analytics (Google Analytics, Plausible, etc.)
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
    console.log('%c✅ Arxis landing page initialized successfully!', 'color: #4CAF50; font-weight: bold;');
});

// ===================================
// Service Worker (PWA - Optional)
// ===================================
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        // Uncomment when you have a service worker file
        // navigator.serviceWorker.register('/sw.js')
        //     .then(reg => console.log('Service Worker registered'))
        //     .catch(err => console.log('Service Worker registration failed'));
    });
}

// ===================================
// Copy Code Functionality for Visual Examples
// ===================================
function copyCode(button) {
    const codeBlock = button.closest('.example-code').querySelector('code');
    const text = codeBlock.textContent;

    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.textContent;
        button.textContent = '✓ Copiado!';
        button.style.background = '#00aa00';

        setTimeout(() => {
            button.textContent = originalText;
            button.style.background = '';
        }, 2000);
    }).catch(err => {
        console.error('Erro ao copiar:', err);
        button.textContent = '✗ Erro';
        setTimeout(() => {
            button.textContent = '📋 Copiar';
        }, 2000);
    });
}

// ===================================
// Enhanced SVG Animations for Visual Examples
// ===================================
document.addEventListener('DOMContentLoaded', () => {
    // Wave Pulse Animation - criar múltiplas ondas
    const waveGroup = document.querySelector('.wave-svg .wave-line')?.parentElement;
    if (waveGroup) {
        // Criar ondas adicionais com delays
        for (let i = 1; i <= 3; i++) {
            const pulse = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
            pulse.setAttribute('class', 'wave-pulse');
            pulse.setAttribute('r', '5');
            pulse.setAttribute('fill', '#FFD700');
            pulse.style.animationDelay = `${i * 1.3}s`;
            waveGroup.appendChild(pulse);
        }
    }

    // Anomaly Detection - animar ponto de anomalia
    const anomalyPoint = document.querySelector('.anomaly-svg .anomaly-point');
    if (anomalyPoint) {
        let scale = 1;
        let growing = true;
        setInterval(() => {
            if (growing) {
                scale += 0.05;
                if (scale >= 1.5) growing = false;
            } else {
                scale -= 0.05;
                if (scale <= 1) growing = true;
            }
            anomalyPoint.setAttribute('r', 6 * scale);
        }, 100);
    }

    // Black Hole - adicionar efeito de distorção visual
    const blackholeCircle = document.querySelector('.blackhole-svg circle[fill="#000"]');
    if (blackholeCircle) {
        let opacity = 0.8;
        let direction = -1;
        setInterval(() => {
            opacity += direction * 0.02;
            if (opacity <= 0.6 || opacity >= 1) direction *= -1;
            blackholeCircle.setAttribute('opacity', opacity);
        }, 50);
    }

    // Tesseract - adicionar rotação suave adicional
    const tesseract = document.querySelector('.tesseract');
    if (tesseract) {
        let angle = 0;
        setInterval(() => {
            angle += 0.5;
            tesseract.style.transform = `rotate(${angle}deg)`;
        }, 50);
    }

    // ===================================
    // Animações Interativas para Novos Exemplos
    // ===================================

    // Série Temporal - animar ponto de previsão
    const forecastLine = document.querySelector('.forecast-data');
    if (forecastLine) {
        let dashOffset = 800;
        setInterval(() => {
            dashOffset -= 2;
            if (dashOffset < 0) dashOffset = 800;
            forecastLine.style.strokeDashoffset = dashOffset;
        }, 50);
    }

    // Transformada de Fourier - animar barras de frequência
    const fourierBars = document.querySelectorAll('.fourier-svg rect[fill*="#4CAF50"], .fourier-svg rect[fill*="#FFD700"], .fourier-svg rect[fill*="#FF8C00"]');
    fourierBars.forEach((bar, index) => {
        const originalHeight = bar.getAttribute('height');
        let growing = true;
        let currentHeight = parseFloat(originalHeight);

        setInterval(() => {
            if (growing) {
                currentHeight += 0.5;
                if (currentHeight >= parseFloat(originalHeight) * 1.1) growing = false;
            } else {
                currentHeight -= 0.5;
                if (currentHeight <= parseFloat(originalHeight) * 0.9) growing = true;
            }
            bar.setAttribute('height', currentHeight);
            bar.setAttribute('y', parseFloat(bar.getAttribute('y')) + (parseFloat(originalHeight) - currentHeight));
        }, 100 + index * 50);
    });

    // Conv4D - animar frames de tempo
    const timeFrames = document.querySelectorAll('.time-frame');
    timeFrames.forEach((frame, index) => {
        let scale = 1;
        let growing = true;

        setInterval(() => {
            if (growing) {
                scale += 0.01;
                if (scale >= 1.1) growing = false;
            } else {
                scale -= 0.01;
                if (scale <= 0.95) growing = true;
            }

            const currentTransform = frame.getAttribute('transform');
            const baseTranslate = currentTransform.match(/translate\(([^)]+)\)/)[1];
            frame.setAttribute('transform', `translate(${baseTranslate}) scale(${scale})`);
        }, 100 + index * 200);
    });

    // Compressão - animar transferência de dados
    const compressionArrows = document.querySelectorAll('.compression-svg path');
    compressionArrows.forEach(arrow => {
        let opacity = 0.7;
        let direction = 1;

        setInterval(() => {
            opacity += direction * 0.03;
            if (opacity >= 1 || opacity <= 0.3) direction *= -1;
            arrow.setAttribute('opacity', opacity);
        }, 100);
    });

    // Hover interativo nos exemplos visuais
    const visualExamples = document.querySelectorAll('.visual-example');
    visualExamples.forEach(example => {
        const viz = example.querySelector('.example-viz');

        example.addEventListener('mouseenter', () => {
            viz.style.transform = 'scale(1.02)';
            viz.style.transition = 'transform 0.3s ease';
        });

        example.addEventListener('mouseleave', () => {
            viz.style.transform = 'scale(1)';
        });
    });
});

