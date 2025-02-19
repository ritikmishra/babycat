#!/usr/bin/env python3
import babycat


def main():
    try:
        waveform = babycat.FloatWaveform.from_file("audio-for-tests/circus-of-freaks/track.mp3")
    except (FileNotFoundError, babycat.exceptions.BabycatError) as exc:
        print("Decoding error:", exc)
        return
    print(
        f"Decoded {waveform.num_frames} frames with "
        f"{waveform.num_channels} channels at "
        f"{waveform.frame_rate_hz} hz"
    )


if __name__ == "__main__":
    main()
