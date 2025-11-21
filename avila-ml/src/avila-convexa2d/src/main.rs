//! # Avila Convexa2D - Exemplo Completo
//!
//! Demonstração de processamento de dados bidimensionais (2D)
//! para imagens e matrizes usando a biblioteca avila-convexa2d

use avila_convexa2d::{
    Image, ImageProcessor, Pixel, ColorSpace,
    Matrix2D, MatrixOps,
    Filter, EdgeDetection, ConvolutionKernel2D,
    Transform2D, Interpolation,
    Point2D, Size2D, Rect,
};
use ndarray::Array2;

fn main() {
    println!("🖼️  Avila Convexa2D - Processamento de Dados Bidimensionais\n");

    // ========================================
    // PARTE 1: Operações Básicas de Imagem
    // ========================================
    println!("📊 PARTE 1: Operações Básicas de Imagem");
    println!("=========================================\n");

    // Cria imagem de teste
    println!("Criando imagem de teste...");
    let mut image = Image::new(100, 100, ColorSpace::RGB);

    // Desenha gradiente
    for y in 0..100 {
        for x in 0..100 {
            let intensity = ((x + y) as f32 / 200.0 * 255.0) as u8;
            image.set_pixel(x, y, Pixel::rgb(intensity, 100, 255 - intensity));
        }
    }
    println!("✓ Imagem criada: {}x{} pixels\n", image.width(), image.height());

    // Operações de pixel
    println!("Operações de pixel:");
    let pixel = image.get_pixel(50, 50);
    println!("  Pixel (50, 50): RGB({}, {}, {})", pixel.r, pixel.g, pixel.b);
    println!("  Em escala de cinza: {}\n", pixel.to_gray());

    // Conversão para escala de cinza
    println!("Convertendo para escala de cinza...");
    let gray = image.to_grayscale();
    println!("✓ Convertido para {} canal\n", gray.channels());

    // Recorte de região
    println!("Recortando região central...");
    let rect = Rect::new(25, 25, 50, 50);
    if let Some(cropped) = image.crop(rect) {
        println!("✓ Região recortada: {}x{}\n", cropped.width(), cropped.height());
    }

    // Ajustes básicos
    println!("Aplicando ajustes de imagem...");
    let bright = ImageProcessor::brightness(&image, 50);
    let contrast = ImageProcessor::contrast(&image, 1.5);
    let inverted = ImageProcessor::invert(&image);
    println!("✓ Brilho ajustado (+50)");
    println!("✓ Contraste ajustado (1.5x)");
    println!("✓ Cores invertidas\n");

    // Binarização
    println!("Aplicando binarização (threshold=128)...");
    let binary = ImageProcessor::threshold(&gray, 128);
    println!("✓ Imagem binarizada\n");

    // ========================================
    // PARTE 2: Operações de Matriz
    // ========================================
    println!("\n📐 PARTE 2: Álgebra de Matrizes");
    println!("=====================================\n");

    // Cria matrizes
    println!("Criando matrizes de teste...");
    let mat1 = Matrix2D::from_array(
        Array2::from_shape_vec((3, 3), vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap()
    );
    println!("✓ Matriz 3x3 criada");

    let mat2 = Matrix2D::from_array(
        Array2::from_shape_vec((3, 3), vec![
            9.0, 8.0, 7.0,
            6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        ]).unwrap()
    );
    println!("✓ Segunda matriz 3x3 criada\n");

    // Operações matriciais
    println!("Operações matriciais:");
    let sum = mat1.add(&mat2).unwrap();
    println!("  Soma: shape = {:?}", sum.shape());

    let sub = mat1.sub(&mat2).unwrap();
    println!("  Subtração: shape = {:?}", sub.shape());

    let scaled = mat1.scale(2.0);
    println!("  Escala (2x): shape = {:?}", scaled.shape());

    let transposed = mat1.transpose();
    println!("  Transposta: {:?} → {:?}", mat1.shape(), transposed.shape());

    println!("  Soma total: {:.2}", mat1.sum());
    println!("  Média: {:.2}\n", mat1.mean());

    // Normalização
    println!("Normalizando matriz...");
    let normalized = mat1.normalize();
    println!("✓ Matriz normalizada (0.0 - 1.0)");
    println!("  Valor mínimo: {:.2}", normalized.data().iter().copied().fold(f32::INFINITY, f32::min));
    println!("  Valor máximo: {:.2}\n", normalized.data().iter().copied().fold(f32::NEG_INFINITY, f32::max));

    // Identidade
    println!("Criando matriz identidade 4x4...");
    let identity = Matrix2D::<f32>::identity(4);
    println!("✓ Matriz identidade criada: {:?}\n", identity.shape());

    // ========================================
    // PARTE 3: Convolução e Filtros
    // ========================================
    println!("\n🎨 PARTE 3: Convolução e Filtros");
    println!("=====================================\n");

    // Filtros de desfoque
    println!("Aplicando filtros de desfoque...");
    let blurred = Filter::blur(&image);
    println!("✓ Desfoque simples aplicado: {}x{}", blurred.width(), blurred.height());

    let gaussian = Filter::gaussian_blur(&image, 3);
    println!("✓ Desfoque gaussiano 3x3 aplicado: {}x{}", gaussian.width(), gaussian.height());

    let gaussian5 = Filter::gaussian_blur(&image, 5);
    println!("✓ Desfoque gaussiano 5x5 aplicado: {}x{}\n", gaussian5.width(), gaussian5.height());

    // Nitidez e realce
    println!("Aplicando filtros de nitidez...");
    let sharpened = Filter::sharpen(&image);
    println!("✓ Nitidez aplicada: {}x{}", sharpened.width(), sharpened.height());

    let edges = Filter::edge_enhance(&image);
    println!("✓ Realce de bordas aplicado: {}x{}\n", edges.width(), edges.height());

    // Detecção de bordas
    println!("Detectando bordas...");
    let sobel = Filter::detect_edges_sobel(&image);
    println!("✓ Sobel aplicado: {}x{}", sobel.width(), sobel.height());

    let laplacian = Filter::detect_edges_laplacian(&image);
    println!("✓ Laplaciano aplicado: {}x{}\n", laplacian.width(), laplacian.height());

    // Kernels customizados
    println!("Criando kernels customizados...");
    let blur_kernel = ConvolutionKernel2D::blur_3x3();
    let sharpen_kernel = ConvolutionKernel2D::sharpen();
    println!("✓ Kernel de desfoque: {:?}", blur_kernel.weights.shape());
    println!("✓ Kernel de nitidez: {:?}\n", sharpen_kernel.weights.shape());

    // ========================================
    // PARTE 4: Transformações Geométricas
    // ========================================
    println!("\n🔄 PARTE 4: Transformações Geométricas");
    println!("=========================================\n");

    // Redimensionamento
    println!("Redimensionando imagem...");
    let resized_nn = Transform2D::resize(&image, 150, 150, Interpolation::NearestNeighbor);
    println!("✓ Nearest Neighbor: {}x{} → {}x{}",
        image.width(), image.height(), resized_nn.width(), resized_nn.height());

    let resized_bi = Transform2D::resize(&image, 50, 50, Interpolation::Bilinear);
    println!("✓ Bilinear: {}x{} → {}x{}\n",
        image.width(), image.height(), resized_bi.width(), resized_bi.height());

    // Escala proporcional
    println!("Escala proporcional...");
    let scaled = Transform2D::scale_proportional(&image, 75);
    println!("✓ Escalado mantendo proporção: {}x{}\n", scaled.width(), scaled.height());

    // Rotação
    println!("Rotacionando imagem...");
    let rotated_45 = Transform2D::rotate(&image, 45.0, Pixel::black());
    println!("✓ Rotação 45°: {}x{}", rotated_45.width(), rotated_45.height());

    let rotated_90 = Transform2D::rotate(&image, 90.0, Pixel::white());
    println!("✓ Rotação 90°: {}x{}\n", rotated_90.width(), rotated_90.height());

    // Flip
    println!("Espelhando imagem...");
    let flipped_h = Transform2D::flip_horizontal(&image);
    let flipped_v = Transform2D::flip_vertical(&image);
    println!("✓ Flip horizontal: {}x{}", flipped_h.width(), flipped_h.height());
    println!("✓ Flip vertical: {}x{}\n", flipped_v.width(), flipped_v.height());

    // Translação
    println!("Transladando imagem...");
    let translated = Transform2D::translate(&image, 10, -10, Pixel::gray(128));
    println!("✓ Translação (10, -10): {}x{}\n", translated.width(), translated.height());

    // ========================================
    // PARTE 5: Geometria 2D
    // ========================================
    println!("\n📍 PARTE 5: Geometria 2D");
    println!("=====================================\n");

    // Pontos
    println!("Operações com pontos:");
    let p1 = Point2D::new(0, 0);
    let p2 = Point2D::new(30, 40);
    println!("  P1: ({}, {})", p1.x, p1.y);
    println!("  P2: ({}, {})", p2.x, p2.y);
    println!("  Distância euclidiana: {:.2}", p1.distance(&p2));
    println!("  Distância Manhattan: {}\n", p1.manhattan_distance(&p2));

    // Tamanhos
    println!("Operações com tamanhos:");
    let size1 = Size2D::new(100, 50);
    let size2 = Size2D::new(200, 100);
    println!("  Size1: {}x{}", size1.width, size1.height);
    println!("  Área: {}", size1.area());
    println!("  Aspecto: {:.2}", size1.aspect_ratio());
    println!("  Cabe em Size2? {}\n", size1.fits_in(&size2));

    // Retângulos
    println!("Operações com retângulos:");
    let rect1 = Rect::new(10, 10, 50, 50);
    let rect2 = Rect::new(30, 30, 50, 50);
    let rect3 = Rect::new(100, 100, 20, 20);
    println!("  Rect1: ({}, {}) {}x{}", rect1.x, rect1.y, rect1.width, rect1.height);
    println!("  Área: {}", rect1.area());
    println!("  Centro: ({}, {})", rect1.center().x, rect1.center().y);
    println!("  Contém (25, 25)? {}", rect1.contains(&Point2D::new(25, 25)));
    println!("  Intersecta Rect2? {}", rect1.intersects(&rect2));
    println!("  Intersecta Rect3? {}\n", rect1.intersects(&rect3));

    // ========================================
    // PARTE 6: Pipeline Completo
    // ========================================
    println!("\n🔬 PARTE 6: Pipeline de Processamento");
    println!("=========================================\n");

    println!("Executando pipeline completo:");
    println!("  1. Imagem original: {}x{}", image.width(), image.height());

    let step1 = image.to_grayscale();
    println!("  2. Converter para grayscale: {}x{}", step1.width(), step1.height());

    let step2 = Filter::gaussian_blur(&step1, 3);
    println!("  3. Aplicar blur gaussiano: {}x{}", step2.width(), step2.height());

    let step3 = Filter::detect_edges_sobel(&step2);
    println!("  4. Detectar bordas (Sobel): {}x{}", step3.width(), step3.height());

    let step4 = ImageProcessor::threshold(&step3, 50);
    println!("  5. Binarizar (threshold): {}x{}", step4.width(), step4.height());

    let step5 = Transform2D::resize(&step4, 64, 64, Interpolation::Bilinear);
    println!("  6. Redimensionar para 64x64: {}x{}", step5.width(), step5.height());

    println!("\n✅ Pipeline concluído!\n");

    // ========================================
    // PARTE 7: Estatísticas Finais
    // ========================================
    println!("📊 ESTATÍSTICAS FINAIS");
    println!("=====================================\n");

    println!("Capacidades da biblioteca:");
    println!("  ✓ Operações básicas de imagem (pixel, crop, convert)");
    println!("  ✓ Ajustes (brilho, contraste, inversão, threshold)");
    println!("  ✓ Álgebra de matrizes (add, sub, scale, transpose)");
    println!("  ✓ Convolução 2D e filtros (blur, sharpen, edge)");
    println!("  ✓ Detecção de bordas (Sobel, Prewitt, Laplacian)");
    println!("  ✓ Transformações geométricas (resize, rotate, flip)");
    println!("  ✓ Interpolação (NN, Bilinear, Bicubic)");
    println!("  ✓ Geometria 2D (Point, Size, Rect)");
    println!("  ✓ Pipelines de processamento completos\n");

    println!("✅ Demonstração completa!");
    println!("\n🚀 Pronto para processar imagens e matrizes 2D com Rust!");
}
