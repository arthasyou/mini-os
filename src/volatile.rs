pub struct Volatile<T>(T);

impl<T: Copy> Volatile<T> {
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(&self.0) }
    }

    #[inline(always)]
    pub fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(&mut self.0, value) }
    }
}
