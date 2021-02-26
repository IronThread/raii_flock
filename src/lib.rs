//! Little library implementing a wrapper over a file that's locked on creation and unlocked when
//! it goes out of scope.

use ::{
        fs2::FileExt,
        std::{
            fs::File,
            io::prelude::*,
            ops::{Deref, DerefMut},
        },
};

/// Wrapper over a file that calls [`FileExt::unlock`] at [dropping][`Drop`].
#[derive(Debug)]
pub struct FileLock(pub File);

impl FileLock {
    /// Creates a `Self` instance calling [`FileExt::try_lock_shared`] on `f` and returning any
    /// error that could have caused.
    pub fn try_wrap_shared(f: File) -> io::Result<Self> {
        f.try_lock_shared()?;
        Ok(Self(f))
    }

    /// Creates a `Self` instance calling [`FileExt::lock_shared`] on `f` and returning any
    /// error that could have caused.
    pub fn wrap_shared(f: File) -> io::Result<Self> {
        f.lock_shared()?;
        Ok(Self(f))
    }

    /// Creates a `Self` instance calling [`FileExt::try_lock_exclusive`] on `f` and returning any
    /// error that could have caused.
    pub fn try_wrap_exclusive(f: File) -> io::Result<Self> {
        f.try_lock_exclusive()?;
        Ok(Self(f))
    }

    /// Creates a `Self` instance calling [`FileExt::lock_exclusive`] on `f` and returning any
    /// error that could have caused.
    pub fn wrap_exclusive(f: File) -> io::Result<Self> {
        f.lock_exclusive()?;
        Ok(Self(f))
    }
}

impl Write for FileLock {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<'a> Write for &'a FileLock {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.0).write(buf)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        (&self.0).flush()
    }
}

impl Read for FileLock {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'a> Read for &'a FileLock {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.0).read(buf)
    }
}

impl Seek for FileLock {
    #[inline(always)]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl<'a> Seek for &'a FileLock {
    #[inline(always)]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        (&self.0).seek(pos)
    }
}

impl Deref for FileLock {
    type Target = File;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileLock {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        if let Err(e) = self.0.unlock() {
            eprintln!("error file lock: {}", e)
        }
    }
}