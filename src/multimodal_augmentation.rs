/// Multi-Modal Data Augmentation for Phase 2
///
/// Generate synthetic data across multiple modalities: vision, audio, sensor, text, temporal

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Modality {
    Vision,   // Images, video, point clouds
    Audio,    // Speech, sound, acoustic
    Sensor,   // LiDAR, IMU, thermal, depth
    Text,     // Natural language, transcripts
    Temporal, // Time series, trajectories
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalSample {
    pub sample_id: String,
    pub modality: String,
    pub data_format: String,
    pub resolution: Option<String>, // e.g., "1024x768" for vision
    pub sample_rate: Option<u32>,   // e.g., 16000 Hz for audio
    pub duration_ms: Option<u32>,
    pub metadata: HashMap<String, String>,
    pub augmentations_applied: Vec<String>,
}

impl MultiModalSample {
    pub fn new(sample_id: String, modality: Modality, format: String) -> Self {
        MultiModalSample {
            sample_id,
            modality: format!("{:?}", modality).to_lowercase(),
            data_format: format,
            resolution: None,
            sample_rate: None,
            duration_ms: None,
            metadata: HashMap::new(),
            augmentations_applied: Vec::new(),
        }
    }
}

pub struct VisionAugmentor;

impl VisionAugmentor {
    /// Generate augmented vision samples
    pub fn generate(base_image: &str, augmentation_types: &[&str]) -> Vec<MultiModalSample> {
        let mut samples = Vec::new();

        for aug_type in augmentation_types {
            let sample_id = format!("vision_{}_{}", base_image, aug_type);
            let mut sample = MultiModalSample::new(sample_id, Modality::Vision, "png".to_string());

            sample.resolution = Some("1024x768".to_string());
            sample.augmentations_applied.push(aug_type.to_string());

            match *aug_type {
                "rotation" => sample.metadata.insert("rotation_degrees".to_string(), "45".to_string()),
                "brightness" => sample.metadata.insert("brightness_factor".to_string(), "1.5".to_string()),
                "blur" => sample.metadata.insert("blur_sigma".to_string(), "2.0".to_string()),
                "crop" => sample.metadata.insert("crop_ratio".to_string(), "0.8".to_string()),
                "noise" => sample.metadata.insert("noise_level".to_string(), "0.1".to_string()),
                _ => None,
            };

            samples.push(sample);
        }

        samples
    }

    /// Get common vision augmentation types
    pub fn available_augmentations() -> Vec<&'static str> {
        vec!["rotation", "brightness", "blur", "crop", "noise", "flip", "color_jitter", "perspective"]
    }
}

pub struct AudioAugmentor;

impl AudioAugmentor {
    /// Generate augmented audio samples
    pub fn generate(base_audio: &str, augmentation_types: &[&str]) -> Vec<MultiModalSample> {
        let mut samples = Vec::new();

        for aug_type in augmentation_types {
            let sample_id = format!("audio_{}_{}", base_audio, aug_type);
            let mut sample = MultiModalSample::new(sample_id, Modality::Audio, "wav".to_string());

            sample.sample_rate = Some(16000); // 16 kHz
            sample.duration_ms = Some(3000);  // 3 seconds
            sample.augmentations_applied.push(aug_type.to_string());

            match *aug_type {
                "pitch_shift" => sample.metadata.insert("pitch_shift_semitones".to_string(), "3".to_string()),
                "time_stretch" => sample.metadata.insert("stretch_factor".to_string(), "1.2".to_string()),
                "noise_injection" => sample.metadata.insert("snr_db".to_string(), "20".to_string()),
                "eq_low" => sample.metadata.insert("low_freq_boost_db".to_string(), "6".to_string()),
                "eq_high" => sample.metadata.insert("high_freq_boost_db".to_string(), "6".to_string()),
                _ => None,
            };

            samples.push(sample);
        }

        samples
    }

    pub fn available_augmentations() -> Vec<&'static str> {
        vec!["pitch_shift", "time_stretch", "noise_injection", "eq_low", "eq_high", "reverb", "compression"]
    }
}

pub struct SensorAugmentor;

impl SensorAugmentor {
    /// Generate augmented sensor data (LiDAR, IMU, thermal, depth)
    pub fn generate(base_sensor: &str, sensor_type: &str, augmentation_types: &[&str]) -> Vec<MultiModalSample> {
        let mut samples = Vec::new();

        for aug_type in augmentation_types {
            let sample_id = format!("sensor_{}_{}", base_sensor, aug_type);
            let mut sample = MultiModalSample::new(
                sample_id,
                Modality::Sensor,
                format!("{}_pcd", sensor_type),
            );

            sample.sample_rate = Some(30);    // 30 Hz
            sample.duration_ms = Some(1000);  // 1 second
            sample.augmentations_applied.push(aug_type.to_string());
            sample.metadata.insert("sensor_type".to_string(), sensor_type.to_string());

            match *aug_type {
                "noise" => sample.metadata.insert("noise_std_dev".to_string(), "0.05".to_string()),
                "dropout" => sample.metadata.insert("dropout_percent".to_string(), "10".to_string()),
                "rotation" => sample.metadata.insert("rotation_axis".to_string(), "z".to_string()),
                "translation" => sample.metadata.insert("translation_magnitude".to_string(), "0.1".to_string()),
                "scale" => sample.metadata.insert("scale_factor".to_string(), "1.05".to_string()),
                _ => None,
            };

            samples.push(sample);
        }

        samples
    }

    pub fn available_augmentations() -> Vec<&'static str> {
        vec!["noise", "dropout", "rotation", "translation", "scale", "occlusion", "temporal_jitter"]
    }
}

pub struct TemporalAugmentor;

impl TemporalAugmentor {
    /// Generate augmented time series data
    pub fn generate(base_series: &str, augmentation_types: &[&str]) -> Vec<MultiModalSample> {
        let mut samples = Vec::new();

        for aug_type in augmentation_types {
            let sample_id = format!("temporal_{}_{}", base_series, aug_type);
            let mut sample = MultiModalSample::new(sample_id, Modality::Temporal, "csv".to_string());

            sample.sample_rate = Some(100); // 100 Hz
            sample.duration_ms = Some(10000);
            sample.augmentations_applied.push(aug_type.to_string());

            match *aug_type {
                "time_warp" => sample.metadata.insert("warp_factor".to_string(), "0.9".to_string()),
                "magnitude_warp" => sample.metadata.insert("magnitude_scale".to_string(), "1.1".to_string()),
                "jittering" => sample.metadata.insert("jitter_std_dev".to_string(), "0.01".to_string()),
                "scaling" => sample.metadata.insert("scaling_factor".to_string(), "1.05".to_string()),
                "rotation" => sample.metadata.insert("rotation_degrees".to_string(), "10".to_string()),
                _ => None,
            };

            samples.push(sample);
        }

        samples
    }

    pub fn available_augmentations() -> Vec<&'static str> {
        vec!["time_warp", "magnitude_warp", "jittering", "scaling", "rotation", "permutation", "window_slicing"]
    }
}

pub struct MultiModalAugmentationPipeline {
    vision_enabled: bool,
    audio_enabled: bool,
    sensor_enabled: bool,
    temporal_enabled: bool,
}

impl MultiModalAugmentationPipeline {
    pub fn new() -> Self {
        MultiModalAugmentationPipeline {
            vision_enabled: true,
            audio_enabled: true,
            sensor_enabled: true,
            temporal_enabled: true,
        }
    }

    /// Generate all modalities for a dataset
    pub fn generate_all(&self, num_samples: usize) -> Vec<MultiModalSample> {
        let mut all_samples = Vec::new();

        if self.vision_enabled {
            for i in 0..num_samples {
                let augmentations = vec!["rotation", "brightness", "noise"];
                let samples = VisionAugmentor::generate(&format!("base_{}", i), &augmentations);
                all_samples.extend(samples);
            }
        }

        if self.audio_enabled {
            for i in 0..num_samples {
                let augmentations = vec!["pitch_shift", "time_stretch", "noise_injection"];
                let samples = AudioAugmentor::generate(&format!("base_{}", i), &augmentations);
                all_samples.extend(samples);
            }
        }

        if self.sensor_enabled {
            for i in 0..num_samples {
                let augmentations = vec!["noise", "rotation", "translation"];
                let samples = SensorAugmentor::generate(&format!("base_{}", i), "lidar", &augmentations);
                all_samples.extend(samples);
            }
        }

        if self.temporal_enabled {
            for i in 0..num_samples {
                let augmentations = vec!["time_warp", "jittering", "scaling"];
                let samples = TemporalAugmentor::generate(&format!("base_{}", i), &augmentations);
                all_samples.extend(samples);
            }
        }

        all_samples
    }

    /// Get statistics about generated data
    pub fn get_generation_stats(&self, samples: &[MultiModalSample]) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        for sample in samples {
            *stats.entry(sample.modality.clone()).or_insert(0) += 1;
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multimodal_sample_creation() {
        let sample = MultiModalSample::new("s1".to_string(), Modality::Vision, "png".to_string());
        assert_eq!(sample.modality, "vision");
        assert_eq!(sample.data_format, "png");
    }

    #[test]
    fn test_vision_augmentation() {
        let samples = VisionAugmentor::generate("base.jpg", &["rotation", "brightness"]);
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].augmentations_applied[0], "rotation");
    }

    #[test]
    fn test_audio_augmentation() {
        let samples = AudioAugmentor::generate("base.wav", &["pitch_shift", "time_stretch"]);
        assert_eq!(samples.len(), 2);
        assert!(samples[0].sample_rate.is_some());
    }

    #[test]
    fn test_sensor_augmentation() {
        let samples = SensorAugmentor::generate("base.pcd", "lidar", &["noise", "rotation"]);
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].metadata.get("sensor_type").unwrap(), "lidar");
    }

    #[test]
    fn test_temporal_augmentation() {
        let samples = TemporalAugmentor::generate("base.csv", &["time_warp", "jittering"]);
        assert_eq!(samples.len(), 2);
        assert!(samples[0].duration_ms.is_some());
    }

    #[test]
    fn test_pipeline_generation() {
        let pipeline = MultiModalAugmentationPipeline::new();
        let samples = pipeline.generate_all(1);

        let stats = pipeline.get_generation_stats(&samples);
        assert!(stats.contains_key("vision"));
        assert!(stats.contains_key("audio"));
    }
}
