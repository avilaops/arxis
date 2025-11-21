# 🛠️ Guia de Extensões - Próximos Passos

Este guia mostra como estender o sistema de reconhecimento facial com funcionalidades adicionais.

---

## 1. 📷 Carregar Imagens Reais

### Adicionar suporte a arquivos JPEG/PNG

```rust
// Em main.rs
use image::io::Reader as ImageReader;

fn load_face_from_file(path: &str) -> Result<Array2<f32>, Box<dyn Error>> {
    let img = ImageReader::open(path)?
        .decode()?
        .to_luma8();

    let (width, height) = img.dimensions();
    let mut face = Array2::zeros((height as usize, width as usize));

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];
            face[[y as usize, x as usize]] = pixel as f32 / 255.0;
        }
    }

    Ok(face)
}

// Uso:
fn main() {
    let face = load_face_from_file("face.jpg").unwrap();
    let features = extract_all_features(&face);
    // ...
}
```

---

## 2. 🎨 Visualização com Plotters

### Plotar Eigenfaces

```rust
use plotters::prelude::*;

fn plot_eigenface(eigenface: &Array1<f32>, width: usize, height: usize, path: &str)
    -> Result<(), Box<dyn std::error::Error>>
{
    let root = BitMapBackend::new(path, (width as u32, height as u32))
        .into_drawing_area();
    root.fill(&WHITE)?;

    // Normaliza para [0, 255]
    let min_val = eigenface.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_val = eigenface.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let val = eigenface[idx];
            let normalized = ((val - min_val) / (max_val - min_val) * 255.0) as u8;

            root.draw_pixel((x as i32, y as i32), &RGBColor(normalized, normalized, normalized))?;
        }
    }

    root.present()?;
    Ok(())
}

// Plotar múltiplas eigenfaces
fn plot_all_eigenfaces(recognizer: &FaceRecognizer, output_dir: &str) {
    if let Some(eigenfaces) = &recognizer.eigenfaces {
        for (i, eigenface) in eigenfaces.axis_iter(Axis(0)).enumerate() {
            let path = format!("{}/eigenface_{}.png", output_dir, i);
            plot_eigenface(&eigenface.to_owned(), 64, 64, &path).unwrap();
        }
    }
}
```

### Plotar Histogramas de Features

```rust
fn plot_hog_histogram(hog: &[f32], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("HOG Features", ("Arial", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..hog.len(), 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.filled())
            .data(hog.iter().enumerate().map(|(i, &v)| (i, v)))
    )?;

    root.present()?;
    Ok(())
}
```

---

## 3. 🎥 Detecção Facial com Viola-Jones

### Implementar cascade classifier

```rust
use imageproc::haar::detect_faces;

fn detect_and_recognize(image_path: &str, recognizer: &FaceRecognizer) {
    let img = image::open(image_path).unwrap().to_luma8();

    // Carrega cascade (Haar features)
    let cascade = haar::load_cascade("haarcascade_frontalface_default.xml").unwrap();

    // Detecta faces
    let faces = detect_faces(&img, &cascade, 1.1, 3, 0);

    for face_rect in faces {
        // Extrai região
        let face_img = imageops::crop_imm(
            &img,
            face_rect.x,
            face_rect.y,
            face_rect.width,
            face_rect.height
        ).to_image();

        // Redimensiona para 64x64
        let face_resized = imageops::resize(&face_img, 64, 64, imageops::FilterType::Lanczos3);

        // Converte para Array2
        let face_array = image_to_array(&face_resized);

        // Reconhece
        let (person_id, confidence) = recognizer.recognize(&face_array);

        println!("Face detectada: Pessoa {} (confiança: {:.1}%)",
                 person_id, confidence * 100.0);
    }
}
```

---

## 4. 🌐 API REST com Actix-web

### Criar servidor de reconhecimento

```rust
// Adicionar ao Cargo.toml:
// actix-web = "4.4"
// tokio = { version = "1", features = ["full"] }

use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};

struct AppState {
    recognizer: Arc<Mutex<FaceRecognizer>>,
}

async fn recognize_face(
    data: web::Data<AppState>,
    bytes: web::Bytes,
) -> HttpResponse {
    // Decodifica imagem
    let img = image::load_from_memory(&bytes).unwrap();
    let gray = img.to_luma8();

    // Converte para array
    let face_array = image_to_array(&gray);

    // Reconhece
    let recognizer = data.recognizer.lock().unwrap();
    let (person_id, confidence) = recognizer.recognize(&face_array);

    HttpResponse::Ok().json(serde_json::json!({
        "person_id": person_id,
        "confidence": confidence,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let recognizer = Arc::new(Mutex::new(FaceRecognizer::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                recognizer: recognizer.clone(),
            }))
            .route("/recognize", web::post().to(recognize_face))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Cliente (Python)

```python
import requests

with open('face.jpg', 'rb') as f:
    response = requests.post('http://localhost:8080/recognize', data=f.read())
    result = response.json()
    print(f"Pessoa: {result['person_id']}, Confiança: {result['confidence']:.2%}")
```

---

## 5. 🧠 Deep Learning com ONNX Runtime

### Usar modelo pré-treinado (FaceNet)

```rust
// Adicionar ao Cargo.toml:
// ort = "2.0"

use ort::{Environment, SessionBuilder, Value};

fn extract_deep_features(face: &Array2<f32>) -> Vec<f32> {
    // Carrega modelo FaceNet
    let environment = Environment::builder().build().unwrap();
    let session = SessionBuilder::new(&environment)
        .unwrap()
        .with_model_from_file("facenet.onnx")
        .unwrap();

    // Prepara input (1, 1, 160, 160)
    let input = face.clone().insert_axis(Axis(0)).insert_axis(Axis(0));
    let input_tensor = Value::from_array(session.allocator(), &input).unwrap();

    // Inferência
    let outputs = session.run(vec![input_tensor]).unwrap();

    // Extrai embeddings (512D)
    outputs[0].extract_tensor::<f32>().unwrap().to_vec()
}
```

---

## 6. 📊 Benchmark e Otimização

### Medir performance

```rust
use std::time::Instant;

fn benchmark_recognition() {
    let mut recognizer = FaceRecognizer::new();

    // Adiciona 100 pessoas, 5 amostras cada
    for person_id in 0..100 {
        for _ in 0..5 {
            let face = create_synthetic_face_for_person(64, 64, person_id);
            recognizer.add_face(person_id, face);
        }
    }

    // Treina
    let start = Instant::now();
    recognizer.train_pca(50);
    println!("Treinamento: {:?}", start.elapsed());

    // Testa reconhecimento
    let test_face = create_synthetic_face_for_person(64, 64, 42);

    let start = Instant::now();
    let (id, conf) = recognizer.recognize(&test_face);
    println!("Reconhecimento: {:?}", start.elapsed());
    println!("Resultado: pessoa {}, confiança {:.2}", id, conf);
}
```

### Paralelizar com Rayon

```rust
use rayon::prelude::*;

fn extract_features_parallel(images: &[Array2<f32>]) -> Vec<Vec<f32>> {
    images.par_iter()
        .map(|img| extract_all_features(img))
        .collect()
}
```

---

## 7. 💾 Persistência de Modelo

### Salvar/carregar modelo treinado

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SerializableModel {
    eigenfaces: Vec<Vec<f32>>,
    mean_face: Vec<f32>,
    database: HashMap<usize, Vec<Vec<f32>>>,
}

impl FaceRecognizer {
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let model = SerializableModel {
            eigenfaces: self.eigenfaces.as_ref()
                .map(|e| e.outer_iter().map(|row| row.to_vec()).collect())
                .unwrap_or_default(),
            mean_face: self.mean_face.as_ref()
                .map(|m| m.to_vec())
                .unwrap_or_default(),
            database: self.database.iter()
                .map(|(k, v)| (*k, v.iter().map(|arr| arr.to_vec()).collect()))
                .collect(),
        };

        let file = File::create(path)?;
        serde_json::to_writer(file, &model)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let model: SerializableModel = serde_json::from_reader(file)?;

        // Reconstruir FaceRecognizer...
        Ok(recognizer)
    }
}
```

---

## 8. 🎯 Interface Gráfica com eGUI

### GUI interativa

```rust
// Adicionar ao Cargo.toml:
// eframe = "0.24"
// egui = "0.24"

use eframe::egui;

struct FaceRecognitionApp {
    recognizer: FaceRecognizer,
    selected_image: Option<Array2<f32>>,
    result: Option<(usize, f32)>,
}

impl eframe::App for FaceRecognitionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sistema de Reconhecimento Facial");

            if ui.button("Carregar Imagem").clicked() {
                // Abrir file dialog
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.selected_image = load_face_from_file(path.to_str().unwrap()).ok();
                }
            }

            if let Some(face) = &self.selected_image {
                if ui.button("Reconhecer").clicked() {
                    self.result = Some(self.recognizer.recognize(face));
                }
            }

            if let Some((person_id, confidence)) = self.result {
                ui.label(format!("Pessoa: {}", person_id));
                ui.label(format!("Confiança: {:.1}%", confidence * 100.0));
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Face Recognition",
        options,
        Box::new(|_cc| Box::new(FaceRecognitionApp::default())),
    )
}
```

---

## 9. 📹 Captura em Tempo Real com OpenCV

### Webcam feed

```rust
// Usar opencv-rust
// opencv = "0.88"

use opencv::{
    prelude::*,
    videoio::{self, VideoCapture, CAP_ANY},
    highgui,
};

fn real_time_recognition() {
    let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();
    let mut recognizer = FaceRecognizer::new();
    // ... treinar recognizer ...

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame).unwrap();

        // Converte para grayscale
        let mut gray = Mat::default();
        opencv::imgproc::cvt_color(&frame, &mut gray, opencv::imgproc::COLOR_BGR2GRAY, 0).unwrap();

        // Detecta faces
        let faces = detect_faces_opencv(&gray);

        for face_rect in faces {
            // Extrai e reconhece
            let face = extract_face(&gray, &face_rect);
            let (person_id, confidence) = recognizer.recognize(&face);

            // Desenha retângulo e texto
            opencv::imgproc::rectangle(
                &mut frame,
                face_rect,
                opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2, 8, 0
            ).unwrap();

            let text = format!("ID: {} ({:.0}%)", person_id, confidence * 100.0);
            opencv::imgproc::put_text(
                &mut frame,
                &text,
                opencv::core::Point::new(face_rect.x, face_rect.y - 10),
                opencv::imgproc::FONT_HERSHEY_SIMPLEX,
                0.5,
                opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                1, 8, false
            ).unwrap();
        }

        highgui::imshow("Face Recognition", &frame).unwrap();

        if highgui::wait_key(1).unwrap() == 27 { // ESC
            break;
        }
    }
}
```

---

## 10. 🔐 Sistema de Controle de Acesso

### Aplicação completa

```rust
struct AccessControlSystem {
    recognizer: FaceRecognizer,
    authorized_ids: HashSet<usize>,
    access_log: Vec<AccessEvent>,
}

struct AccessEvent {
    person_id: usize,
    confidence: f32,
    granted: bool,
    timestamp: SystemTime,
}

impl AccessControlSystem {
    fn check_access(&mut self, face: &Array2<f32>) -> bool {
        let (person_id, confidence) = self.recognizer.recognize(face);

        let granted = self.authorized_ids.contains(&person_id)
                      && confidence > 0.85;

        self.access_log.push(AccessEvent {
            person_id,
            confidence,
            granted,
            timestamp: SystemTime::now(),
        });

        granted
    }

    fn generate_report(&self) -> String {
        // Gera relatório de acessos
        format!("Total acessos: {}\nAutorizados: {}\nNegados: {}",
            self.access_log.len(),
            self.access_log.iter().filter(|e| e.granted).count(),
            self.access_log.iter().filter(|e| !e.granted).count())
    }
}
```

---

## 📚 Recursos Adicionais

### Datasets Públicos
- **LFW** (Labeled Faces in the Wild)
- **CelebA** (200k+ faces de celebridades)
- **VGGFace2** (3.3M imagens)
- **MS-Celeb-1M**

### Modelos Pré-treinados
- **FaceNet** (Google)
- **ArcFace** (InsightFace)
- **VGGFace**
- **OpenFace**

### Ferramentas
- **dlib** - C++ library com Python bindings
- **face_recognition** - Python library simples
- **MTCNN** - Detecção multi-task
- **RetinaFace** - State-of-the-art detection

---

## 🎯 Roadmap Sugerido

1. **Semana 1-2**: Carregar imagens reais + visualização
2. **Semana 3-4**: Detecção facial (Viola-Jones)
3. **Semana 5-6**: API REST + persistência
4. **Semana 7-8**: Deep learning (ONNX)
5. **Semana 9-10**: Interface gráfica
6. **Semana 11-12**: Tempo real (webcam)

---

**Boa sorte com suas extensões!** 🚀
