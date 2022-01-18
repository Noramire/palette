pub trait Quantization {
    fn build(&mut self);
    fn get_palette(&mut self) -> Vec<String>;
    fn to_buffer(&mut self) -> Vec<u8>;
}
