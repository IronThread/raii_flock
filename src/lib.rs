//! Little library implementing a wrapper over a file that's locked on creation and unlocked when
//! it goes out of scope.

use ::{
        fs2::FileExt,
        std::{
            fs::File,
            io::{self, SeekFrom, prelude::*},
            ops::{Deref, DerefMut},
            thread::panicking,
        },
};

/// Wrapper over a file that calls [`FileExt::unlock`] at [dropping][`Drop`].
#[derive(Debug)]
pub struct FileLock<'a>(pub &'a File);

impl<'a> FileLock<'a> {
    /// Creates a `Self` instance calling [`FileExt::try_lock_shared`] on `f` and returning any
    /// error that could have caused.
    pub fn try_wrap_shared(f: &'a File) -> io::Result<Self> {
        f.try_lock_shared()?;
        Ok(Self(f))
    }

    /// Creates a `Self` instance calling [`FileExt::lock_shared`] on `f` and returning any
    /// error that could have caused.
    pub fn wrap_shared(f: &'a File) -> io::Result<Self> {
        f.lock_shared()?;
        Ok(Self(f))
    }

    /// Creates a `Self` instance calling [`FileExt::try_lock_exclusive`] on `f` and returning any
    /// error that could have caused.
    pub fn try_wrap_exclusive(f: &'a File) -> io::Result<Self> {
        f.try_lock_exclusive()?;
        Ok(Self(f))
    }

    /// Creates a `Self` instance calling [`FileExt::lock_exclusive`] on `f` and returning any
    /// error that could have caused.
    pub fn wrap_exclusive(f: &'a File) -> io::Result<Self> {
        f.lock_exclusive()?;
        Ok(Self(f))
    }
}

impl<'a> Write for FileLock<'a> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<'a> Read for FileLock<'a> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'a> Seek for FileLock<'a> {
    #[inline(always)]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl<'a> Deref for FileLock<'a> {
    type Target = &'a File;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for FileLock<'a> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Drop for FileLock<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.0.unlock() {
            if panicking() {
                eprintln!("error unlocking file lock: {}", e)
            } else {
                panic!("error unlocking file lock: {}", e)
            }
        }
    }
}