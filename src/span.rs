/// Structure for storing byte index positions inside the source code
/// [`u32`] value stores the starting index
/// [`u16`] stores the length form said index
/// This makes the maximum token length 65535 characters
#[derive(Debug, Clone, Copy)]
pub struct Span(u32, u16);
impl Span {
    pub fn init(start: usize, len: usize) -> Self {
        Span(start as u32, len as u16)
    }
    pub fn start(&self) -> usize {
        self.0 as usize
    }
    pub fn start_u32(&self) -> u32 {
        self.0  
    }
    pub fn len(&self) -> usize {
        self.1 as usize
    }
    pub fn end(&self) -> usize {
        self.0 as usize + self.1 as usize
    }
    pub fn set1(&mut self, curr: usize) {
        self.0 = curr as u32;
        self.1 = 1;
    }
}
