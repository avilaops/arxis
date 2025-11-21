//! # Avila Convexa1D - Exemplo Completo
//!
//! Demonstração de processamento de dados sequenciais 1D
//! para áudio e texto usando a biblioteca avila-convexa1d

use avila_convexa1d::{
    AudioProcessor, AudioFeatures,
    TextProcessor, TextFeatures,
    ConvolutionKernel, SequentialData,
};

fn main() {
    println!("🎵 Avila Convexa1D - Processamento de Dados Sequenciais\n");

    // ========================================
    // PARTE 1: Processamento de Áudio
    // ========================================
    println!("📊 PARTE 1: Processamento de Áudio");
    println!("=====================================\n");

    let audio_processor = AudioProcessor::default();

    // Gera sinal de teste: tom de 440Hz (Lá) por 0.5 segundos
    println!("Gerando sinal de áudio (440Hz, 0.5s)...");
    let audio_signal = audio_processor.generate_sine_wave(440.0, 0.5, 0.8);
    println!("✓ Sinal gerado: {} amostras", audio_signal.len());

    // Adiciona ruído branco
    let noisy_signal = audio_processor.add_white_noise(&audio_signal, 0.05);
    println!("✓ Ruído adicionado (5%)\n");

    // Extrai features do áudio
    println!("Extraindo features de áudio...");
    let audio_features = audio_processor.extract_features(&noisy_signal);
    print_audio_features(&audio_features);

    // Aplica filtro de suavização (convolução)
    println!("\nAplicando filtro gaussiano...");
    let kernel = ConvolutionKernel::gaussian(64, 10.0);
    let filtered = audio_processor.temporal_convolution(&noisy_signal, &kernel);
    println!("✓ Sinal filtrado: {} amostras", filtered.len());

    let filtered_features = audio_processor.extract_features(&filtered);
    println!("  Energia antes: {:.4}", audio_features.energy);
    println!("  Energia depois: {:.4}", filtered_features.energy);

    // Cria sequência de dados estruturada
    let audio_seq = SequentialData::new(
        noisy_signal.to_vec(),
        audio_processor.sample_rate,
    ).with_metadata("Tom de 440Hz com ruído".to_string());

    println!("\n✓ Sequência de áudio criada:");
    println!("  Duração: {:.2}s", audio_seq.duration());
    println!("  Taxa de amostragem: {}Hz", audio_seq.sample_rate);

    // ========================================
    // PARTE 2: Processamento de Texto
    // ========================================
    println!("\n\n📝 PARTE 2: Processamento de Texto");
    println!("=====================================\n");

    let mut text_processor = TextProcessor::new(16);

    // Corpus de exemplo
    let corpus = vec![
        "AvilaDB é o banco de dados nativo do Brasil".to_string(),
        "Processamento de linguagem natural com Rust".to_string(),
        "Machine learning e deep learning com dados sequenciais".to_string(),
        "Áudio e texto são dados sequenciais unidimensionais".to_string(),
        "Convolução temporal para extração de features".to_string(),
    ];

    println!("Construindo vocabulário...");
    text_processor.build_vocab(&corpus);
    println!("✓ Vocabulário construído: {} tokens únicos\n", text_processor.vocab.len());

    // Processa cada texto
    for (i, text) in corpus.iter().enumerate() {
        println!("Texto {}: \"{}\"", i + 1, text);

        // Tokeniza
        let tokens = text_processor.tokenize(text);
        println!("  Tokens: {:?}", tokens);

        // Converte para índices
        let indices = text_processor.text_to_indices(text);
        println!("  Índices: {:?}", indices);

        // Extrai features
        let features = text_processor.extract_features(text);
        print_text_features(&features);

        println!();
    }

    // Análise temporal de texto
    println!("Aplicando convolução temporal em texto...");
    let sample_text = "AvilaDB é o banco de dados nativo do Brasil com baixa latência";
    let text_kernel = ConvolutionKernel::moving_average(3);

    if let Some(convolved) = text_processor.temporal_convolution(sample_text, &text_kernel) {
        println!("✓ Convolução aplicada:");
        println!("  Shape: {}x{}", convolved.nrows(), convolved.ncols());
        println!("  Embedding médio pós-convolução: {:.4}",
                 convolved.mean().unwrap_or(0.0));
    }

    // ========================================
    // PARTE 3: Comparação e Análise
    // ========================================
    println!("\n\n🔬 PARTE 3: Análise Comparativa");
    println!("=====================================\n");

    println!("Características dos dados sequenciais:\n");

    println!("ÁUDIO:");
    println!("  • Alta taxa de amostragem (44.1kHz)");
    println!("  • Natureza contínua e temporal");
    println!("  • Features: energia, ZCR, MFCC, envelope");
    println!("  • Análise espectral importante\n");

    println!("TEXTO:");
    println!("  • Sequência discreta de tokens");
    println!("  • Embeddings de alta dimensão");
    println!("  • Features: densidade lexical, comprimento");
    println!("  • Análise semântica e sintática\n");

    println!("CONVOLUÇÃO 1D:");
    println!("  • Extração de padrões locais");
    println!("  • Invariância temporal");
    println!("  • Redução de dimensionalidade");
    println!("  • Filtro de ruído e suavização\n");

    // ========================================
    // PARTE 4: Serialização
    // ========================================
    println!("\n📦 PARTE 4: Serialização de Features");
    println!("=====================================\n");

    if let Ok(json) = audio_features.to_json() {
        println!("Features de áudio (JSON):");
        println!("{}\n", json);
    }

    let text_features_vec = text_processor.extract_features(&corpus[0]);
    println!("Features de texto (vetor):");
    println!("{:?}\n", text_features_vec.to_vector());

    println!("✅ Demonstração completa!");
    println!("\n🚀 Pronto para processar dados sequenciais com Rust!");
}

/// Imprime features de áudio formatadas
fn print_audio_features(features: &AudioFeatures) {
    println!("  📊 Features extraídas:");
    println!("    • Duração: {:.3}s", features.duration);
    println!("    • Energia: {:.4}", features.energy);
    println!("    • ZCR: {:.4}", features.zcr);
    println!("    • RMS: {:.4}", features.rms);
    println!("    • Média: {:.4}", features.mean);
    println!("    • Desvio padrão: {:.4}", features.std);
    println!("    • Amplitude máxima: {:.4}", features.max_amplitude);
    println!("    • Envelope médio: {:.4}", features.envelope_mean);
    println!("    • MFCC ({} coefs): [{:.3}, {:.3}, ...]",
             features.mfcc.len(),
             features.mfcc.get(0).unwrap_or(&0.0),
             features.mfcc.get(1).unwrap_or(&0.0));
}

/// Imprime features de texto formatadas
fn print_text_features(features: &TextFeatures) {
    println!("  📊 Features:");
    println!("    • Tokens: {}", features.token_count);
    println!("    • Únicos: {}", features.unique_tokens);
    println!("    • Comprimento médio: {:.2}", features.avg_token_length);
    println!("    • Densidade lexical: {:.3}", features.lexical_density);
    println!("    • Comprimento sequência: {}", features.sequence_length);
}
