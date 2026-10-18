let category = AVAudioSessionCategoryPlayback
    .ok_or_else(|| anyhow::anyhow!("Playback audio category is unavailable"))?;

session.setCategory_withOptions_error(
    category,
    AVAudioSessionCategoryOptions::empty(),
)
