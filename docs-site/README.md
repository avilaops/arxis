# Arxis Documentation Site

Official documentation website for Arxis - The Mathematical Citadel.

## 🌐 Live Site

**URL**: https://docs.avilaops.com

## 📦 Structure

```
docs-site/
├── index.html          # Homepage
├── styles.css          # Global styles
├── script.js           # Interactive features
├── CNAME               # Custom domain configuration
├── modules/            # Module documentation pages
├── api/                # API reference pages
├── tutorials/          # Step-by-step tutorials
├── examples/           # Code examples
├── guides/             # Best practices guides
└── assets/             # Images, icons, fonts
```

## 🚀 Deployment

### GitHub Pages

1. Push to repository:
```bash
git add docs-site/
git commit -m "Add documentation site"
git push origin main
```

2. Configure GitHub Pages:
   - Go to repository Settings
   - Navigate to Pages section
   - Source: Deploy from a branch
   - Branch: `main`
   - Folder: `/docs-site`
   - Save

3. Configure custom domain:
   - Add CNAME record in DNS:
     ```
     docs.avilaops.com -> avilaops.github.io
     ```
   - GitHub will automatically detect the CNAME file

### Local Development

```bash
# Simple HTTP server (Python)
cd docs-site
python -m http.server 8000

# Or with Node.js
npx serve .

# Open browser
open http://localhost:8000
```

## 📝 Adding Content

### New Module Documentation

Create `modules/your-module.html`:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Your Module - Arxis Documentation</title>
    <link rel="stylesheet" href="../styles.css">
</head>
<body>
    <!-- Navigation -->
    <!-- Content -->
    <!-- Footer -->
</body>
</html>
```

### New Tutorial

Create `tutorials/your-tutorial.html` following the same structure.

### New API Reference

Create `api/your-api.html` with detailed API documentation.

## 🎨 Styling

The site uses:
- **Primary color**: `#00d4ff` (Cyan)
- **Secondary color**: `#FFD700` (Gold)
- **Accent color**: `#CE422B` (Rust orange)
- **Dark theme**: Gradient background

All styles are in `styles.css` - modify CSS variables in `:root` to change theme.

## 🔧 Features

- ✅ Responsive design (mobile, tablet, desktop)
- ✅ Smooth scrolling navigation
- ✅ Active link highlighting
- ✅ Animated cards on scroll
- ✅ Copy code buttons
- ✅ Dark theme optimized
- ✅ SEO friendly

## 📞 Contact

- **Email**: nicolas@avila.inc
- **GitHub**: https://github.com/avilaops/arxis
- **Organization**: https://avilaops.com

---

Built with ❤️ by Avila
