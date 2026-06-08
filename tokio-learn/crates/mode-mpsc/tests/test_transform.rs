use mode_mpsc::transform::{TransformationType, transform};

#[tokio::test]
async fn test_transform_mp4_to_wav() {
    let result = transform(
        "tests/test.mp4",
        "tests/test.wav",
        TransformationType::Vidoe2Wav,
    )
    .await;
    assert!(result.is_ok(), "transform failed: {:?}", result.err());
}
