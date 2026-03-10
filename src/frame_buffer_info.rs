#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorField {
    pub position: u8,
    pub size: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramebufferInfo {
    pub address: usize,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bits_per_pixel: u8,
    pub red: ColorField,
    pub green: ColorField,
    pub blue: ColorField,
    pub alpha: ColorField,
}

impl FramebufferInfo {
    // +-------+----------------+
    // | idx   | description    |
    // +-------+----------------+
    // | [ 0]  | address        |
    // | [ 1]  | width          |
    // | [ 2]  | height         |
    // | [ 3]  | stride         |
    // | [ 4]  | bpp            |
    // | [ 5]  | red.position   |
    // | [ 6]  | red.size       |
    // | [ 7]  | green.position |
    // | [ 8]  | greeb.size     |
    // | [ 9]  | blue.position  |
    // | [10]  | blue.size      |
    // | [11]  | alpha.position  |
    // | [12]  | alpha.size      |
    // +-------+----------------+
    pub fn serialize(&self) -> [usize; 13] {
        [
            self.address,
            self.width as usize,
            self.height as usize,
            self.stride as usize,
            self.bits_per_pixel as usize,
            self.red.position as usize,
            self.red.size as usize,
            self.green.position as usize,
            self.green.size as usize,
            self.blue.position as usize,
            self.blue.size as usize,
            self.alpha.position as usize,
            self.alpha.size as usize,
        ]
    }

    pub fn deserialize(data: &[usize; 13]) -> Self {
        FramebufferInfo {
            address: data[0],
            width: data[1] as u32,
            height: data[2] as u32,
            stride: data[3] as u32,
            bits_per_pixel: data[4] as u8,
            red: ColorField {
                position: data[5] as u8,
                size: data[6] as u8,
            },
            green: ColorField {
                position: data[7] as u8,
                size: data[8] as u8,
            },
            blue: ColorField {
                position: data[9] as u8,
                size: data[10] as u8,
            },
            alpha: ColorField {
                position: data[11] as u8,
                size: data[12] as u8,
            },
        }
    }
}
