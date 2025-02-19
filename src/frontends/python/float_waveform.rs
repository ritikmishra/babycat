use crate::backend::Waveform;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::PyObjectProtocol;

/// A container for decoding operations that may have succeeded or failed.
#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct FloatWaveformNamedResult {
    /// The "name" of a result as a :py:class:`str`, typically a filename for an audio file.
    #[pyo3(get)]
    pub name: String,
    /// A :py:class:`~babycat.FloatWaveform` if decoding succeeded... or ``None`` if decoding failed.
    #[pyo3(get)]
    pub waveform: Option<FloatWaveform>,
    error: Option<crate::backend::Error>,
}

#[pymethods]
impl FloatWaveformNamedResult {
    /// ``None`` if decoding succeeded... or an exception if decoding failed.
    #[getter]
    fn get_exception(&self) -> Option<PyErr> {
        match self.error {
            Some(error) => Some(PyErr::from(error)),
            None => None,
        }
    }
}

impl From<crate::backend::NamedResult<crate::backend::FloatWaveform, crate::backend::Error>>
    for FloatWaveformNamedResult
{
    fn from(
        inner: crate::backend::NamedResult<crate::backend::FloatWaveform, crate::backend::Error>,
    ) -> Self {
        match inner.result {
            Ok(waveform) => Self {
                name: inner.name,
                waveform: Some(waveform.into()),
                error: None,
            },
            Err(err) => Self {
                name: inner.name,
                waveform: None,
                error: Some(err),
            },
        }
    }
}

impl std::fmt::Display for FloatWaveformNamedResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.waveform {
            Some(waveform) => {
                write!(
                    f,
                    "<babycat.FloatWaveformNamedResult: name={} waveform={}>",
                    self.name, waveform
                )
            }
            None => match self.error {
                Some(error) => {
                    write!(
                        f,
                        "<babycat.FloatWaveformNamedResult name={} error={}>",
                        self.name,
                        error.to_string()
                    )
                }
                None => {
                    write!(f, "<babycat.FloatWaveformNamedResult name={}>", self.name,)
                }
            },
        }
    }
}

#[pyproto]
impl PyObjectProtocol for FloatWaveformNamedResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

/// An in-memory audio waveform.
#[pyclass(module = "babycat")]
#[derive(Clone, Debug)]
pub struct FloatWaveform {
    inner: crate::backend::FloatWaveform,
}

impl From<crate::backend::FloatWaveform> for FloatWaveform {
    fn from(inner: crate::backend::FloatWaveform) -> FloatWaveform {
        FloatWaveform { inner }
    }
}

impl std::fmt::Display for FloatWaveform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<babycat.FloatWaveform: {} frames, {} channels, {} hz>",
            self.inner.num_frames(),
            self.inner.num_channels(),
            self.inner.frame_rate_hz(),
        )
    }
}

#[pyproto]
impl PyObjectProtocol for FloatWaveform {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

fn float_waveform_to_pyresult(
    result: Result<crate::backend::FloatWaveform, crate::backend::Error>,
) -> PyResult<FloatWaveform> {
    match result {
        Ok(waveform) => Ok(waveform.into()),
        Err(error) => Err(error.into()),
    }
}

#[pymethods]
impl FloatWaveform {
    /// Creates a silent waveform of ``num_frames`` frames.
    ///
    /// Example:
    ///     **Creating 1000 frames of silence (in stereo).**
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> FloatWaveform.from_frames_of_silence(
    ///     ...    frame_rate_hz=44100,
    ///     ...    num_channels=2,
    ///     ...    num_frames=1000,
    ///     ... )
    ///     <babycat.FloatWaveform: 1000 frames, 2 channels, 44100 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The frame rate to set for this silent audio
    ///         waveform.
    ///
    ///     num_channels(int): The number of channels to set.
    ///
    ///     num_frames(int): The number of frames to set.
    ///
    /// Returns:
    ///     FloatWaveform: A waveform representing silence.
    ///
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, num_frames)]
    #[text_signature = "(
        frame_rate_hz,
        num_channels,
        num_frames,
    )"]
    pub fn from_frames_of_silence(frame_rate_hz: u32, num_channels: u32, num_frames: u64) -> Self {
        crate::backend::FloatWaveform::from_frames_of_silence(
            frame_rate_hz,
            num_channels,
            num_frames,
        )
        .into()
    }

    /// Creates a silent waveform of ``duration_milliseconds`` milliseconds.
    ///
    /// Example:
    ///     **Create 30 seconds of silence (in stereo).**
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> FloatWaveform.from_milliseconds_of_silence(
    ///     ...    frame_rate_hz=44100,
    ///     ...    num_channels=2,
    ///     ...    duration_milliseconds=30_000,
    ///     ... )
    ///     <babycat.FloatWaveform: 1323000 frames, 2 channels, 44100 hz>
    ///
    /// Args:
    ///     frame_rate_hz(int): The frame rate to set for this silent audio
    ///         waveform.
    ///
    ///     num_channels(int): The number of channels to set.
    ///
    ///     num_frames(int): The number of frames to set.
    ///
    /// Returns:
    ///     FloatWaveform: A waveform representing silence.
    ///
    #[staticmethod]
    #[args("*", frame_rate_hz, num_channels, duration_milliseconds)]
    #[text_signature = "(
        frame_rate_hz,
        num_channels,
        duration_milliseconds,
    )"]
    pub fn from_milliseconds_of_silence(
        frame_rate_hz: u32,
        num_channels: u32,
        duration_milliseconds: u64,
    ) -> Self {
        crate::backend::FloatWaveform::from_milliseconds_of_silence(
            frame_rate_hz,
            num_channels,
            duration_milliseconds,
        )
        .into()
    }

    /// Decodes audio stored as ``bytes``.
    ///
    /// Example:
    ///     **Decode from bytes while auto-detecting the format as MP3.**
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> with open("audio-for-tests/andreas-theme/track.mp3", "rb") as fh:
    ///     ...     the_bytes = fh.read()
    ///     >>> waveform = FloatWaveform.from_encoded_bytes(the_bytes)
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 9586944 frames, 2 channels, 44100 hz>
    ///
    /// Example:
    ///     **Decode from bytes with a file extension hint.**
    ///
    ///     >>> waveform2 = FloatWaveform.from_encoded_bytes(
    ///     ...     the_bytes,
    ///     ...     file_extension="mp3",
    ///     ... )
    ///
    /// Args:
    ///     encoded_bytes(bytes): A :py:class:`bytes` object
    ///         containing an *encoded* audio file, such as MP3 file.
    ///
    ///     start_time_milliseconds(int, optional): We discard
    ///         any audio before this millisecond offset. By default, this
    ///         does nothing and the audio is decoded from the beginning.
    ///         Negative offsets are invalid.
    ///
    ///     end_time_milliseconds(int, optional): We discard
    ///         any audio after this millisecond offset. By default,
    ///         this does nothing and the audio is decoded all the way
    ///         to the end. If ``start_time_milliseconds`` is specified,
    ///         then ``end_time_milliseconds`` must be greater. The resulting
    ///
    ///     frame_rate_hz(int, optional): A destination frame rate to resample
    ///         the audio to. Do not specify this parameter if you wish
    ///         Babycat to preserve the audio's original frame rate.
    ///         This does nothing if ``frame_rate_hz`` is equal to the
    ///         audio's original frame rate.
    ///
    ///     num_channels(int, optional): Set this to a positive integer ``n``
    ///         to select the *first* ``n`` channels stored in the
    ///         audio file. By default, Babycat will return all of the channels
    ///         in the original audio. This will raise an exception
    ///         if you specify a ``num_channels`` greater than the actual
    ///         number of channels in the audio.
    ///
    ///     convert_to_mono(bool, optional): Set to ``True`` to average all channels
    ///         into a single monophonic (mono) channel. If
    ///         ``num_channels = n`` is also specified, then only the
    ///         first ``n`` channels will be averaged. Note that
    ///         ``convert_to_mono`` cannot be set to ``True`` while
    ///         also setting ``num_channels = 1``.
    ///
    ///     zero_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will zero-pad the ending of the decoded waveform
    ///         to ensure that the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``zero_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``
    ///         if the input audio is shorter than ``end_time_milliseconds``.
    ///
    ///     resample_mode(int, optional): Sets which resampling method
    ///         is used if you have set ``frame_rate_hz``. This
    ///         usually defaults to the highest-accuracy resampler
    ///         compiled into Babycat.
    ///
    ///     file_extension(str, optional): An *optional hint* of the input audio file's
    ///         encoding. An example of a valid value is ``"mp3"``. Babycat
    ///         will automatically detect the correct encoding of ``input_audio``,
    ///         even if ``file_extension`` is an incorrect guess.
    ///
    ///     mime_type(str, optional): An *optional hint* of the input audio file's
    ///         encoding. An example of a valid value is ``"audio/mpeg"``. Babycat
    ///         will automatically detect the correct encoding of ``input_audio``,
    ///         even if ``mime_type`` is an incorrect guess.
    ///
    /// Returns:
    ///     FloatWaveform: Returns a waveform decoded from ``encoded_bytes``.
    ///
    /// Raises:
    ///     babycat.exceptions.FeatureNotCompiled: Raised when you are trying
    ///         to use a feature at runtime that as not included in Babycat
    ///         at compile-time.
    ///
    ///     babycat.exceptions.WrongTimeOffset: Raised when
    ///         ``start_time_milliseconds``and/or ``end_time_milliseconds``
    ///         is invalid.
    ///
    ///     babycat.exceptions.WrongNumChannels: Raised when you specified
    ///         a value for ``num_channels`` that is greater than the
    ///         number of channels the audio has.
    ///
    ///     babycat.exceptions.WrongNumChannelsAndMono: Raised when the
    ///         user sets both ``convert_to_mono = True`` and
    ///         ``num_channels = 1``.
    ///
    ///     babycat.exceptions.CannotZeroPadWithoutSpecifiedLength: Raised
    ///         when ``zero_pad_ending`` is set without setting
    ///         ``end_time_milliseconds``.
    ///
    ///     babycat.exceptions.UnknownInputEncoding: Raised when we
    ///         failed to detect valid audio in the input data.
    ///
    ///     babycat.exceptions.UnknownDecodeError: Raised when we
    ///         failed to decode the input audio stream, but
    ///         we don't know why.
    ///
    ///     babycat.exceptions.ResamplingError: Raised when we
    ///         failed to encode an audio stream into an output format.
    ///
    ///     babycat.exceptions.WrongFrameRate: Raised when the
    ///         user set ``frame_rate_hz`` to a value that we
    ///         cannot resample to.
    ///
    ///     babycat.exceptions.WrongFrameRateRatio: Raised
    ///         when ``frame_rate_hz`` would upsample or
    ///         downsample by a factor ``>= 256``. Try resampling in
    ///         smaller increments.
    ///
    #[staticmethod]
    #[args(
        encoded_bytes,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        resample_mode = 0,
        file_extension = "\"\"",
        mime_type = "\"\""
    )]
    #[text_signature = "(
        encoded_bytes,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        resample_mode = 0,
        file_extension = \"\",
        mime_type = \"\",
    )"]
    #[allow(clippy::too_many_arguments)]
    pub fn from_encoded_bytes(
        encoded_bytes: Vec<u8>,
        start_time_milliseconds: u64,
        end_time_milliseconds: u64,
        frame_rate_hz: u32,
        num_channels: u32,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        resample_mode: u32,
        file_extension: &str,
        mime_type: &str,
    ) -> PyResult<Self> {
        let decode_args = crate::backend::DecodeArgs {
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        };
        float_waveform_to_pyresult(crate::backend::FloatWaveform::from_encoded_bytes_with_hint(
            &encoded_bytes,
            decode_args,
            file_extension,
            mime_type,
        ))
    }

    /// Decodes audio stored in a local file.
    ///
    /// Example:
    ///     **Decode an entire audio file with default arguments.**
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> waveform = FloatWaveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ... )
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 9586944 frames, 2 channels, 44100 hz>
    ///     >>> waveform.num_frames
    ///     9586944
    ///     >>> waveform.num_channels
    ///     2
    ///     >>> waveform.frame_rate_hz
    ///     44100
    ///     >>> waveform.numpy().shape
    ///     (9586944, 2)
    ///
    /// Example:
    ///     **Decode the first 30 seconds of the audio file.**
    ///
    ///     >>> waveform = FloatWaveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ...     end_time_milliseconds=30_000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 1323000 frames, 2 channels, 44100 hz>
    ///
    /// Example:
    ///     **Decode the entire audio file and resampling up to 48,000hz.**
    ///
    ///     >>> waveform = FloatWaveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ...     frame_rate_hz=48000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 10434769 frames, 2 channels, 48000 hz>
    ///
    /// Example:
    ///     **Decode the first 30 seconds and resample up to 48,000hz.**
    ///
    ///     >>> waveform = FloatWaveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ...     end_time_milliseconds=30_000,
    ///     ...     frame_rate_hz=48000,
    ///     ... )
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 1440000 frames, 2 channels, 48000 hz>
    ///
    /// Args:
    ///     filename(str): The path to an audio file on the local
    ///         filesystem.
    ///
    ///     start_time_milliseconds(int, optional): We discard
    ///         any audio before this millisecond offset. By default, this
    ///         does nothing and the audio is decoded from the beginning.
    ///         Negative offsets are invalid.
    ///
    ///     end_time_milliseconds(int, optional): We discard
    ///         any audio after this millisecond offset. By default,
    ///         this does nothing and the audio is decoded all the way
    ///         to the end. If ``start_time_milliseconds`` is specified,
    ///         then ``end_time_milliseconds`` must be greater. The resulting
    ///
    ///     frame_rate_hz(int, optional): A destination frame rate to resample
    ///         the audio to. Do not specify this parameter if you wish
    ///         Babycat to preserve the audio's original frame rate.
    ///         This does nothing if ``frame_rate_hz`` is equal to the
    ///         audio's original frame rate.
    ///
    ///     num_channels(int, optional): Set this to a positive integer ``n``
    ///         to select the *first* ``n`` channels stored in the
    ///         audio file. By default, Babycat will return all of the channels
    ///         in the original audio. This will raise an exception
    ///         if you specify a ``num_channels`` greater than the actual
    ///         number of channels in the audio.
    ///
    ///     convert_to_mono(bool, optional): Set to ``True`` to average all channels
    ///         into a single monophonic (mono) channel. If
    ///         ``num_channels = n`` is also specified, then only the
    ///         first ``n`` channels will be averaged. Note that
    ///         ``convert_to_mono`` cannot be set to ``True`` while
    ///         also setting ``num_channels = 1``.
    ///
    ///     zero_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will zero-pad the ending of the decoded waveform
    ///         to ensure that the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``zero_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``
    ///         if the input audio is shorter than ``end_time_milliseconds``.
    ///
    ///     resample_mode(int, optional): Sets which resampling method
    ///         is used if you have set ``frame_rate_hz``. This
    ///         usually defaults to the highest-accuracy resampler
    ///         compiled into Babycat.
    ///
    /// Returns:
    ///     FloatWaveform: A waveform decoded from ``filename``.
    ///
    /// Raises:
    ///     FileNotFoundError: Raised when we cannot find
    ///         ``filename`` on the local filesystem.
    ///
    ///     IsADirectoryError: Raised when ``filename``
    ///         resolves to a directory on the local
    ///         instead of a file.
    ///
    ///     babycat.exceptions.FeatureNotCompiled: Raised when you are trying
    ///         to use a feature at runtime that as not included in Babycat
    ///         at compile-time.
    ///
    ///     babycat.exceptions.WrongTimeOffset: Raised when
    ///         ``start_time_milliseconds``and/or ``end_time_milliseconds``
    ///         is invalid.
    ///
    ///     babycat.exceptions.WrongNumChannels: Raised when you specified
    ///         a value for ``num_channels`` that is greater than the
    ///         number of channels the audio has.
    ///
    ///     babycat.exceptions.WrongNumChannelsAndMono: Raised when the
    ///         user sets both ``convert_to_mono = True`` and
    ///         ``num_channels = 1``.
    ///
    ///     babycat.exceptions.CannotZeroPadWithoutSpecifiedLength: Raised
    ///         when ``zero_pad_ending`` is set without setting
    ///         ``end_time_milliseconds``.
    ///
    ///     babycat.exceptions.UnknownInputEncoding: Raised when we
    ///         failed to detect valid audio in the input data.
    ///
    ///     babycat.exceptions.UnknownDecodeError: Raised when we
    ///         failed to decode the input audio stream, but
    ///         we don't know why.
    ///
    ///     babycat.exceptions.ResamplingError: Raised when we
    ///         failed to encode an audio stream into an output format.
    ///
    ///     babycat.exceptions.WrongFrameRate: Raised when the
    ///         user set ``frame_rate_hz`` to a value that we
    ///         cannot resample to.
    ///
    ///     babycat.exceptions.WrongFrameRateRatio: Raised
    ///         when ``frame_rate_hz`` would upsample or
    ///         downsample by a factor ``>= 256``. Try resampling in
    ///         smaller increments.
    ///
    #[cfg(feature = "enable-filesystem")]
    #[staticmethod]
    #[args(
        filename,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        resample_mode = 0
    )]
    #[text_signature = "(
        filename,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        resample_mode = 0,
    )"]
    #[allow(clippy::too_many_arguments)]
    pub fn from_file(
        filename: &str,
        start_time_milliseconds: u64,
        end_time_milliseconds: u64,
        frame_rate_hz: u32,
        num_channels: u32,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        resample_mode: u32,
    ) -> PyResult<Self> {
        let decode_args = crate::backend::DecodeArgs {
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        };
        float_waveform_to_pyresult(crate::backend::FloatWaveform::from_file(
            filename,
            decode_args,
        ))
    }

    /// Uses multithreading in Rust to decode many audio files in parallel.
    ///
    ///
    /// Example:
    ///     **(Attempt to) decode three files.**
    ///
    ///     In this example, we succesfully decode two MP3 files with
    ///     the default decoding arguments. Then, we demonstrate
    ///     how to catch an error when decoding the third file.    
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> filenames = [
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ...     "audio-for-tests/blippy-trance/track.mp3",
    ///     ...     "does-not-exist",
    ///     ... ]
    ///     >>>
    ///     >>> batch = FloatWaveform.from_many_files(filenames)
    ///
    ///     The first two files are decoded as expected, with the
    ///     ``exception`` field being ``None`` and the ``waveform``
    ///     field containing a :py:class:`FloatWaveform`.
    ///
    ///     >>> batch[0].name
    ///     'audio-for-tests/andreas-theme/track.mp3'
    ///     >>> print(batch[0].exception)
    ///     None
    ///     >>> batch[0].waveform
    ///     <babycat.FloatWaveform: 9586944 frames, 2 channels, 44100 hz>
    ///
    ///     >>> batch[1].name
    ///     'audio-for-tests/blippy-trance/track.mp3'
    ///     >>> print(batch[1].exception)
    ///     None
    ///     >>> batch[1].waveform
    ///     <babycat.FloatWaveform: 5294592 frames, 2 channels, 44100 hz>
    ///
    ///     For the third file, the ``waveform`` field is ``None`` and the
    ///     ``exception`` field contains a reference to a
    ///     :py:class:`FileNotFoundError`. The ``name`` field helps us
    ///     identify which file is missing.
    ///
    ///     >>> batch[2].name
    ///     'does-not-exist'
    ///     >>> type(batch[2].exception)
    ///     <class 'FileNotFoundError'>
    ///     >>> print(batch[2].waveform)
    ///     None
    ///     >>>
    ///
    ///     .. admonition:: Remember to raise exceptions when needed
    ///         :class: danger
    ///
    ///         :py:meth:`~FloatWaveform.from_many_files` will return
    ///         exceptions but **will not raise them** for you. It is your
    ///         responsibility to check every ``exception`` field for
    ///         a not-``None`` exception that you can raise or
    ///         intentionally ignore.
    ///
    /// Args:
    ///     filenames(list[str]): A :py:class:`list` of filenames--each as
    ///         :py:class:`str`--to decode in parallel.
    ///
    ///     start_time_milliseconds(int, optional): We discard
    ///         any audio before this millisecond offset. By default, this
    ///         does nothing and the audio is decoded from the beginning.
    ///         Negative offsets are invalid.
    ///
    ///     end_time_milliseconds(int, optional): We discard
    ///         any audio after this millisecond offset. By default,
    ///         this does nothing and the audio is decoded all the way
    ///         to the end. If ``start_time_milliseconds`` is specified,
    ///         then ``end_time_milliseconds`` must be greater. The resulting
    ///
    ///     frame_rate_hz(int, optional): A destination frame rate to resample
    ///         the audio to. Do not specify this parameter if you wish
    ///         Babycat to preserve the audio's original frame rate.
    ///         This does nothing if ``frame_rate_hz`` is equal to the
    ///         audio's original frame rate.
    ///
    ///     num_channels(int, optional): Set this to a positive integer ``n``
    ///         to select the *first* ``n`` channels stored in the
    ///         audio file. By default, Babycat will return all of the channels
    ///         in the original audio. This will raise an exception
    ///         if you specify a ``num_channels`` greater than the actual
    ///         number of channels in the audio.
    ///
    ///     convert_to_mono(bool, optional): Set to ``True`` to average all channels
    ///         into a single monophonic (mono) channel. If
    ///         ``num_channels = n`` is also specified, then only the
    ///         first ``n`` channels will be averaged. Note that
    ///         ``convert_to_mono`` cannot be set to ``True`` while
    ///         also setting ``num_channels = 1``.
    ///
    ///     zero_pad_ending(bool, optional): If you set this to ``True``,
    ///         Babycat will zero-pad the ending of the decoded waveform
    ///         to ensure that the output waveform's duration is exactly
    ///         ``end_time_milliseconds - start_time_milliseconds``.
    ///         By default, ``zero_pad_ending = False``, in which case
    ///         the output waveform will be shorter than
    ///         ``end_time_milliseconds - start_time_milliseconds``
    ///         if the input audio is shorter than ``end_time_milliseconds``.
    ///
    ///     resample_mode(int, optional): Sets which resampling method
    ///         is used if you have set ``frame_rate_hz``. This
    ///         usually defaults to the highest-accuracy resampler
    ///         compiled into Babycat.
    ///
    ///     num_workers(int, optional): The number of threads--*Rust threads*, not Python
    ///         threads--to use for parallel decoding of the audio files in
    ///         ``filenames``. By default, Babycat creates the same
    ///         number of threads as the number of logical CPU cores on
    ///         your machine.
    ///
    /// Returns:
    ///     list[FloatWaveformNamedResult]: A list of objects that contain
    ///     a :py:class:`~babycat.FloatWaveform` for every successful encoding
    ///     and a Python exception for every failed encoding. Look at
    ///     the ``"Raises"`` section of :py:meth:`FloatWaveform.decode_from_filename`
    ///     for a list of possible exceptions that can be returned by this method.
    ///
    #[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
    #[staticmethod]
    #[args(
        filenames,
        "*",
        start_time_milliseconds = 0,
        end_time_milliseconds = 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = false,
        zero_pad_ending = false,
        resample_mode = 0,
        num_workers = 0
    )]
    #[text_signature = "(
        filenames,
        start_time_milliseconds = 0,
        end_time_milliseconds= 0,
        frame_rate_hz = 0,
        num_channels = 0,
        convert_to_mono = False,
        zero_pad_ending = False,
        resample_mode = 0,
        num_workers = 0,
    )"]
    #[allow(clippy::too_many_arguments)]
    pub fn from_many_files(
        filenames: Vec<String>,
        start_time_milliseconds: u64,
        end_time_milliseconds: u64,
        frame_rate_hz: u32,
        num_channels: u32,
        convert_to_mono: bool,
        zero_pad_ending: bool,
        resample_mode: u32,
        num_workers: usize,
    ) -> Vec<FloatWaveformNamedResult> {
        let decode_args = crate::backend::DecodeArgs {
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        };
        let batch_args = crate::backend::BatchArgs { num_workers };
        let filenames_ref: Vec<&str> = filenames.iter().map(|f| f.as_str()).collect();
        crate::backend::FloatWaveform::from_many_files(&filenames_ref, decode_args, batch_args)
            .into_iter()
            .map(FloatWaveformNamedResult::from)
            .collect()
    }

    /// Returns the decoded waveform's frame rate in hertz.
    ///
    /// If you did not set ``frame_rate_hz`` as an argument during decoding,
    /// then this value will be the frame rate that Babycat detected from
    /// the audio.
    ///
    /// If you *did* set ``frame_rate_hz`` during decoding, then this value
    /// will be the value you set.
    ///
    /// Returns:
    ///     int: The frame rate.
    ///
    #[getter]
    pub fn get_frame_rate_hz(&self) -> u32 {
        self.inner.frame_rate_hz()
    }

    /// Returns the number of channels in the decoded waveform.
    ///
    /// If you did not set ``num_channels`` as an argument during decoding,
    /// then this value will be the total number of channels found in the audio.
    ///
    /// If you *did* set ``num_channels`` during decoding, then this value
    /// will be the value you set.
    ///
    /// Returns:
    ///     int: The number of channels
    ///
    #[getter]
    pub fn get_num_channels(&self) -> u32 {
        self.inner.num_channels()
    }

    /// Returns the number of frames in the decoded waveform.
    ///
    /// This will be the total number of frames founded in the encoded
    /// audio--unless you trimmed the waveform during decoding by setting
    /// ``start_time_milliseconds``, ``end_time_milliseconds``, or both.
    ///
    /// Returns:
    ///     int: The number of frames
    ///
    #[getter]
    pub fn get_num_frames(&self) -> u64 {
        self.inner.num_frames()
    }

    /// Returns the waveform as a 2D :py:class:`numpy.ndarray` array with shape ``(frames, channels)``
    ///
    /// Babycat internally stores decoded audio as a Rust ``Vec<f32>``.
    /// This method converts the ``Vec<f32>`` into a NumPy array.
    /// Babycat does not internally cache the NumPy array, so avoid
    /// calling this method multiple times on the same
    /// :py:class:`~babycat.FloatWaveform`.
    ///
    /// Babycat is also designed to release the Python Global Interpreter
    /// Lock (GIL) when *decoding* audio into a ``Vec<f32>``, but Babycat
    /// re-acquires the GIL when converting the  the ``Vec<f32>`` into a NumPy array.
    ///
    /// Returns:
    ///     numpy.ndarray: A NumPy array with frames as the first axis
    ///     and channels as the second axis.
    ///
    #[args()]
    #[text_signature = "()"]
    pub fn numpy(&self, py: Python) -> Py<PyArray2<f32>> {
        self.inner
            .interleaved_samples()
            .to_owned()
            .into_pyarray(py)
            .reshape([
                self.inner.num_frames() as usize,
                self.inner.num_channels() as usize,
            ])
            .unwrap()
            .into()
    }

    /// Encodes the waveform into a :py:class:`bytearray` in the WAV format.
    ///
    /// Example:
    ///     **Decode an MP3 file and re-encode it as WAV.**
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> waveform = FloatWaveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ... )
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 9586944 frames, 2 channels, 44100 hz>
    ///     >>> arr = waveform.to_wav_buffer()
    ///     >>> type(arr)
    ///     >>> len(arr)
    ///
    /// Returns:
    ///     bytearray: The encoded WAV file.
    ///
    /// Raises:
    ///     babycat.exceptions.UnknownEncodeError: When something went wrong with the
    ///         encoding.
    ///
    #[args()]
    #[text_signature = "()"]
    pub fn to_wav_buffer(&self, py: Python) -> PyResult<Py<PyAny>> {
        match self.inner.to_wav_buffer() {
            Ok(vec_u8) => Ok((*PyByteArray::new(py, &vec_u8)).to_object(py)),
            Err(err) => Err(PyErr::from(err)),
        }
    }

    /// Writes the waveform to the filesystem as a WAV file.
    ///
    /// Example:
    ///     **Decode an MP3 file and re-encode it as WAV.**
    ///
    ///     >>> from babycat import FloatWaveform
    ///     >>> waveform = FloatWaveform.from_file(
    ///     ...     "audio-for-tests/andreas-theme/track.mp3",
    ///     ... )
    ///     >>> waveform
    ///     <babycat.FloatWaveform: 9586944 frames, 2 channels, 44100 hz>
    ///     >>> waveform.to_wav_file("track.wav")
    ///
    /// Args:
    ///     filename(str): The filename to write the WAV file to.
    ///
    /// Raises:
    ///     babycat.exceptions.UnknownEncodeError: When something went wrong with the
    ///         encoding.
    ///
    #[cfg(feature = "enable-filesystem")]
    #[args(filename)]
    #[text_signature = "(filename)"]
    pub fn to_wav_file(&self, filename: &str) -> PyResult<()> {
        self.inner.to_wav_file(filename).map_err(PyErr::from)
    }

    /// Generates an HTML audio widget in IPython and Jupyter notebooks.
    pub fn _repr_html_(&self) -> PyResult<String> {
        let wav = self.inner.to_wav_buffer()?;
        let wav_buffer_base64 = base64::encode(&wav);
        Ok(format!(
            "
<audio controls>
    <source src='data:audio/wav;base64,{}' type='audio/wav' />
    Your browser does not support the audio element.
</audio>",
            wav_buffer_base64
        ))
    }
}
