//! Configure the iOS output route before CPAL opens RemoteIO.
use anyhow::{anyhow, Result};
use objc2_avf_audio::{AVAudioSession, AVAudioSessionCategoryOptions, AVAudioSessionCategoryPlayback, AVAudioSessionSetActiveOptions};

pub(super) fn activate() -> Result<()> {
    let session = unsafe { AVAudioSession::sharedInstance() };
    unsafe {
        session.setCategory_withOptions_error(AVAudioSessionCategoryPlayback, AVAudioSessionCategoryOptions::empty())
            .map_err(|error| anyhow!("setting playback category: {error:?}"))?;
        session.setActive_withOptions_error(true, AVAudioSessionSetActiveOptions::empty())
            .map_err(|error| anyhow!("activating playback session: {error:?}"))?;
    }
    Ok(())
}
