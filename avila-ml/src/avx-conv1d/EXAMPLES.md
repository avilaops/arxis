# Exemplos de Uso - Avila Convexa1D

## 1. Análise de Áudio Básica

```rust
use avila_convexa1d::AudioProcessor;

fn main() {
    let processor = AudioProcessor::default();

    // Gera tom puro
    let signal = processor.generate_sine_wave(440.0, 2.0, 1.0);

    // Extrai características
    let features = processor.extract_features(&signal);

    println!("Duração: {:.2}s", features.duration);
    println!("Energia: {:.4}", features.energy);
    println!("RMS: {:.4}", features.rms);
}
```

## 2. Filtragem de Áudio

```rust
use avila_convexa1d::{AudioProcessor, ConvolutionKernel};

fn remove_noise(signal: &Array1<f32>) -> Array1<f32> {
    let processor = AudioProcessor::default();

    // Filtro de suavização
    let kernel = ConvolutionKernel::gaussian(128, 15.0);
    processor.temporal_convolution(signal, &kernel)
}
```

## 3. Análise de Texto

```rust
use avila_convexa1d::TextProcessor;

fn analyze_sentiment(text: &str) -> f32 {
    let mut processor = TextProcessor::new(16);

    let corpus = vec![/* seu corpus de treinamento */];
    processor.build_vocab(&corpus);

    let features = processor.extract_features(text);

    // Retorna densidade lexical como proxy de complexidade
    features.lexical_density
}
```

## 4. Comparação de Textos

```rust
use avila_convexa1d::TextProcessor;

fn compare_texts(text1: &str, text2: &str) -> f32 {
    let mut processor = TextProcessor::new(32);

    processor.build_vocab(&[text1.to_string(), text2.to_string()]);

    let emb1 = processor.text_to_embeddings(text1).unwrap();
    let emb2 = processor.text_to_embeddings(text2).unwrap();

    // Calcula similaridade cosseno (simplificado)
    let mean1 = emb1.mean_axis(ndarray::Axis(0)).unwrap();
    let mean2 = emb2.mean_axis(ndarray::Axis(0)).unwrap();

    let dot_product: f32 = mean1.iter().zip(mean2.iter())
        .map(|(a, b)| a * b).sum();

    dot_product
}
```

## 5. Detecção de Eventos em Áudio

```rust
use avila_convexa1d::AudioProcessor;

fn detect_events(signal: &Array1<f32>, threshold: f32) -> Vec<usize> {
    let processor = AudioProcessor::default();
    let envelope = processor.compute_envelope(signal);

    let mut events = Vec::new();
    for (i, &val) in envelope.iter().enumerate() {
        if val > threshold {
            events.push(i);
        }
    }

    events
}
```

## 6. Pipeline Completo

```rust
use avila_convexa1d::{AudioProcessor, TextProcessor, SequentialData};

fn audio_to_text_pipeline() {
    // 1. Processa áudio
    let audio_proc = AudioProcessor::default();
    let audio = audio_proc.generate_sine_wave(440.0, 1.0, 0.8);
    let audio_features = audio_proc.extract_features(&audio);

    // 2. Gera descrição textual
    let description = format!(
        "Áudio de {:.2}s com energia {:.4}",
        audio_features.duration,
        audio_features.energy
    );

    // 3. Processa texto
    let mut text_proc = TextProcessor::new(16);
    text_proc.build_vocab(&[description.clone()]);
    let text_features = text_proc.extract_features(&description);

    println!("Pipeline completo:");
    println!("  Áudio -> {} features", audio_features.to_vector().len());
    println!("  Texto -> {} features", text_features.to_vector().len());
}
```

## 7. Salvando Resultados

```rust
use avila_convexa1d::AudioProcessor;
use std::fs::File;
use std::io::Write;

fn save_features(signal: &Array1<f32>, filename: &str) -> std::io::Result<()> {
    let processor = AudioProcessor::default();
    let features = processor.extract_features(signal);

    let json = features.to_json()?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
```

## 8. Processamento em Batch

```rust
use avila_convexa1d::AudioProcessor;

fn process_batch(signals: Vec<Array1<f32>>) -> Vec<AudioFeatures> {
    let processor = AudioProcessor::default();

    signals.iter()
        .map(|signal| processor.extract_features(signal))
        .collect()
}
```
