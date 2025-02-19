#![allow(dead_code)]

pub const AT_FILENAME: &str = "./audio-for-tests/andreas-theme/track.mp3";
pub const AT_NUM_CHANNELS: u32 = 2;
pub const AT_NUM_FRAMES: u64 = 9586944;
pub const AT_FRAME_RATE_HZ: u32 = 44100;

pub const BT_FILENAME: &str = "./audio-for-tests/blippy-trance/track.mp3";
pub const BT_NUM_CHANNELS: u32 = 2;
pub const BT_NUM_FRAMES: u64 = 5294592;
pub const BT_FRAME_RATE_HZ: u32 = 44100;

pub const COF_FILENAME: &str = "./audio-for-tests/circus-of-freaks/track.mp3";
pub const COF_NUM_CHANNELS: u32 = 2;
pub const COF_NUM_FRAMES: u64 = 2492928;
pub const COF_FRAME_RATE_HZ: u32 = 44100;

pub const LCT_FILENAME: &str = "./audio-for-tests/left-channel-tone/track.mp3";
pub const LCT_NUM_CHANNELS: u32 = 2;
pub const LCT_NUM_FRAMES: u64 = 1325952;
pub const LCT_FRAME_RATE_HZ: u32 = 44100;

pub const MONO_DTMF_FILENAME: &str = "./audio-for-tests/mono-dtmf-tones/track.mp3";
pub const MONO_DTMF_NUM_CHANNELS: u32 = 1;
pub const MONO_DTMF_NUM_FRAMES: u64 = 443520;
pub const MONO_DTMF_FRAME_RATE_HZ: u32 = 44100;

pub const OHFY_FILENAME: &str = "./audio-for-tests/on-hold-for-you/track.mp3";
pub const OHFY_NUM_CHANNELS: u32 = 2;
pub const OHFY_NUM_FRAMES: u64 = 9620352;
pub const OHFY_FRAME_RATE_HZ: u32 = 44100;

pub const TMS_FILENAME: &str = "./audio-for-tests/tone-missing-sounds/track.mp3";
pub const TMS_NUM_CHANNELS: u32 = 1;
pub const TMS_NUM_FRAMES: u64 = 1325952;
pub const TMS_FRAME_RATE_HZ: u32 = 44100;

pub const VR_FILENAME: &str = "./audio-for-tests/voxel-revolution/track.mp3";
pub const VR_NUM_CHANNELS: u32 = 2;
pub const VR_NUM_FRAMES: u64 = 5728896;
pub const VR_FRAME_RATE_HZ: u32 = 44100;

pub const ALL_FILENAMES: &[&str] = &[
    AT_FILENAME,
    BT_FILENAME,
    COF_FILENAME,
    LCT_FILENAME,
    MONO_DTMF_FILENAME,
    OHFY_FILENAME,
    TMS_FILENAME,
    VR_FILENAME,
];

pub const ALL_NUM_CHANNELS: &[u32] = &[
    AT_NUM_CHANNELS,
    BT_NUM_CHANNELS,
    COF_NUM_CHANNELS,
    LCT_NUM_CHANNELS,
    MONO_DTMF_NUM_CHANNELS,
    OHFY_NUM_CHANNELS,
    TMS_NUM_CHANNELS,
    VR_NUM_CHANNELS,
];

pub const ALL_NUM_FRAMES: &[u64] = &[
    AT_NUM_FRAMES,
    BT_NUM_FRAMES,
    COF_NUM_FRAMES,
    LCT_NUM_FRAMES,
    MONO_DTMF_NUM_FRAMES,
    OHFY_NUM_FRAMES,
    TMS_NUM_FRAMES,
    VR_NUM_FRAMES,
];

pub const ALL_FRAME_RATE_HZ: &[u32] = &[
    AT_FRAME_RATE_HZ,
    BT_FRAME_RATE_HZ,
    COF_FRAME_RATE_HZ,
    LCT_FRAME_RATE_HZ,
    MONO_DTMF_FRAME_RATE_HZ,
    OHFY_FRAME_RATE_HZ,
    TMS_FRAME_RATE_HZ,
    VR_FRAME_RATE_HZ,
];
