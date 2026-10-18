//! Configure the iOS output route before CPAL opens RemoteIO.
use anyhow::{anyhow, Result};
use objc2_avf_audio::{
    AVAudioSession,
    AVAudioSessionCategoryOptions,
    AVAudioSessionCategoryPlayback,
    AVAudioSessionSetActiveOptions,
};

pub(super) fn activate() -> Result<()> {
    let session = unsafe { AVAudioSession::sharedInstance() };

    unsafe {
        let category = AVAudioSessionCategoryPlayback
            .ok_or_else(|| anyhow!("Playback audio category is unavailable"))?;

        session
            .setCategory_withOptions_error(
                category,
                AVAudioSessionCategoryOptions::empty(),
            )
            .map_err(|error| anyhow!("setting playback category: {error:?}"))?;

        session
            .setActive_withOptions_error(
                true,
                AVAudioSessionSetActiveOptions::empty(),
            )
            .map_err(|error| anyhow!("activating playback session: {error:?}"))?;
    }

    Ok(())
}
