#[cfg(target_os = "macos")]
mod osx;

#[cfg(target_os = "macos")]
use osx::internal;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::internal;

pub fn microphone_state() -> Option<State> {
    internal::MediaState::new().mic_status()
}

/// NOTE: might run into [this issue](https://github.com/antonfisher/go-media-devices-state/issues/2)
pub fn camera_state() -> Option<State> {
    internal::MediaState::new().cam_state()
}

pub fn apps_using_mic() -> Vec<String> {
    internal::MediaState::apps_using_mic()
}

pub fn apps_using_cam() -> Vec<String> {
    internal::MediaState::apps_using_cam()
}

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

#[cfg(test)]
mod test {
    use crate::{apps_using_cam, apps_using_mic, camera_state, microphone_state};

    // Run any process that uses mic and/or cam and check if the states are accurate
    #[test]
    fn test_mic_cam_state() {
        println!(
            "Mic status: {:?}\nCam status: {:?}\n",
            microphone_state(),
            camera_state()
        );
    }

    #[test]
    fn test_mic_cam_apps() {
        println!(
            "Apps using mic: {:?}\nApps using cam: {:?}\n",
            apps_using_mic(),
            apps_using_cam()
        );
    }
}
