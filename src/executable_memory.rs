pub struct ExecutableMemory {
    pub ptr: *mut core::ffi::c_void,
    pub len: usize,
}

impl ExecutableMemory {
    pub fn new(len: usize) -> Option<Self> {
        let ptr = unsafe {
            libc::mmap(
                core::ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        match ptr {
            libc::MAP_FAILED => None,
            _ => Some(ExecutableMemory { ptr, len }),
        }
    }

    pub fn execute(&self, data: &mut Vec<u8>) -> u32 {
        let dptr = data.as_mut_ptr() as *mut libc::c_char;
        let f: fn(*mut libc::c_char) -> u32 = unsafe { std::mem::transmute(self.ptr) };
        f(dptr)
    }
}

impl std::convert::TryFrom<&[u8]> for ExecutableMemory {
    type Error = ();
    fn try_from(buffer: &[u8]) -> Result<ExecutableMemory, ()> {
        let Some(mut vmem) = ExecutableMemory::new(buffer.len()) else {
            return Err(());
        };
        vmem.copy_from_slice(buffer);
        Ok(vmem)
    }
}

impl core::ops::Deref for ExecutableMemory {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr as *const u8, self.len) }
    }
}
impl core::ops::DerefMut for ExecutableMemory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr as *mut u8, self.len) }
    }
}

impl Drop for ExecutableMemory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, self.len);
        }
    }
}
