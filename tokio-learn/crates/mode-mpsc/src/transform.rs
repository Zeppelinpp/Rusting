use std::process::Stdio;
use tokio::process::Command;

#[derive(Default, Debug)]
pub enum TransformationType {
    #[default]
    Video2MP3,
    Vidoe2Wav,
    Audio2Video,
}

#[derive(Default, Debug)]
pub struct TransformationResult {
    pub in_path: String,
    pub out_path: String,
    pub transformation_type: TransformationType,
}

pub async fn check_ffmpeg() -> bool {
    Command::new("ffmpeg")
        .args(["-version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map_or(false, |s| s.success())
}

pub async fn transform(
    in_path: &str,
    out_path: &str,
    transformation_type: TransformationType,
) -> Result<TransformationResult, String> {
    let output = match transformation_type {
        TransformationType::Vidoe2Wav => {
            Command::new("ffmpeg")
                .args([
                    "-y", // 覆盖输出文件
                    "-i",
                    in_path, // 输入
                    "-vn",   // 去掉视频流
                    "-acodec",
                    "pcm_s16le", // 16bit 小端 PCM
                    "-ar",
                    "44100", // 采样率 44.1kHz
                    "-ac",
                    "2", // 双声道
                    out_path,
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
                .await
                .map_err(|e| e.to_string())?
        }
        TransformationType::Audio2Video => {
            return Err("Audio2Video not implemented".to_string());
        }
        TransformationType::Video2MP3 => {
            return Err("Video2MP3 not implemented".to_string());
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "ffmpeg exit code {:?}: {}",
            output.status.code(),
            stderr.trim()
        ));
    }

    Ok(TransformationResult {
        in_path: in_path.to_string(),
        out_path: out_path.to_string(),
        transformation_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_check_ffmpeg() {
        assert!(check_ffmpeg().await);
    }

    #[tokio::test]
    async fn test_transform_one_video() {
        let in_path = "tests/test.mp4";
        let out_path = "tests/test.wav";
        assert!(
            transform(in_path, out_path, TransformationType::Vidoe2Wav)
                .await
                .is_ok()
        )
    }
}
