/// A wrapper for [`std::result::Result`][std::result::Result] that names each individual result.
///
/// Babycat returns a [`std::vec::Vec`][std::vec::Vec] of [`NamedResult`][crate::NamedResult] structs
/// from any function that operates on a batch
/// of many inputs, where each operation could raise a separate error.
///
/// For example,
/// [`FloatWaveform::from_many_files()`][crate::FloatWaveform#method.from_many_files]
/// reads a list of audio files and decodes them in parallel. For each input,
/// [`from_many_files()`][crate::FloatWaveform#method.from_many_files] returns a
/// [`NamedResult`][crate::NamedResult] containing the input filename and a
/// [`std::result::Result`] that contains a
/// [`FloatWaveform`][crate::FloatWaveform] if decoding succeeded or a
/// [`Error`][crate::Error] if decoding failed.
/// This allows you to track the decoding state of each input file with
/// minimal bookkeeping on your end.

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NamedResult<T, E> {
    pub name: String,
    pub result: Result<T, E>,
}
