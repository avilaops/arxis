# Arxis Landing Page Assets

Este diretório contém todos os assets estáticos da landing page.

## 📁 Estrutura Requerida

```
assets/
├── favicon.svg           # Favicon SVG (inline no HTML)
├── favicon.ico           # Favicon ICO multi-resolution
├── og-image.png          # Open Graph image (1200×630)
├── twitter-card.png      # Twitter card image (1200×600)
├── icon-192.png          # PWA icon (192×192)
├── icon-512.png          # PWA icon (512×512)
└── README.md             # Este arquivo
```

## 🎨 Criação de Assets

### Favicon.ico

Criar usando ferramenta online:
- [favicon.io](https://favicon.io/)
- [realfavicongenerator.net](https://realfavicongenerator.net/)

Incluir tamanhos: 16×16, 32×32, 48×48

### OG Image (1200×630)

Design em Figma/Canva:
- Background: #0A1628
- Logo quaternion spiral centralizado
- Texto: "ARXIS - Research-Grade Physics in Rust"
- Gradiente cyan→purple

### Twitter Card (1200×600)

Similar ao OG image, proporções diferentes.

### PWA Icons

Exportar logo em:
- 192×192px (icon-192.png)
- 512×512px (icon-512.png)

## 🔗 Links Úteis

- **Favicon Generator**: https://favicon.io/
- **OG Image Generator**: https://www.opengraph.xyz/
- **Image Optimizer**: https://tinypng.com/

## ✅ Checklist

- [ ] favicon.svg criado
- [ ] favicon.ico criado (multi-res)
- [ ] og-image.png criado (1200×630)
- [ ] twitter-card.png criado (1200×600)
- [ ] icon-192.png criado
- [ ] icon-512.png criado
- [ ] Todos otimizados (TinyPNG)
- [ ] Assets commitados no Git
