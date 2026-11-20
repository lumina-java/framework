/// ESC/POS Thermal Receipt Printer Command Builder
#[derive(Debug, Clone, Default)]
pub struct EscPosBuilder {
    buffer: Vec<u8>,
}

impl EscPosBuilder {
    pub fn new() -> Self {
        let mut builder = Self { buffer: Vec::new() };
        builder.initialize();
        builder
    }

    /// ESC @ - Initialize printer
    pub fn initialize(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x40]);
        self
    }

    /// Set text alignment: Left
    pub fn align_left(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x00]);
        self
    }

    /// Set text alignment: Center
    pub fn align_center(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x01]);
        self
    }

    /// Set text alignment: Right
    pub fn align_right(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x02]);
        self
    }

    /// Set bold text
    pub fn bold(&mut self, enable: bool) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x45, if enable { 1 } else { 0 }]);
        self
    }

    /// Set double-height and double-width font (e.g. for store header)
    pub fn double_size(&mut self, enable: bool) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1D, 0x21, if enable { 0x11 } else { 0x00 }]);
        self
    }

    /// Set underline
    pub fn underline(&mut self, enable: bool) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x2D, if enable { 1 } else { 0 }]);
        self
    }

    /// Append plain text with newline
    pub fn line(&mut self, text: &str) -> &mut Self {
        self.buffer.extend_from_slice(text.as_bytes());
        self.buffer.push(b'\n');
        self
    }

    /// Append plain text without newline
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.buffer.extend_from_slice(text.as_bytes());
        self
    }

    /// Print horizontal separator rule (e.g. "--------------------------------")
    pub fn separator(&mut self, width: usize) -> &mut Self {
        let line = "-".repeat(width);
        self.line(&line)
    }

    /// Format two columns (left-aligned title, right-aligned value) for receipts
    pub fn two_columns(&mut self, left: &str, right: &str, width: usize) -> &mut Self {
        let total_chars = left.chars().count() + right.chars().count();
        if total_chars < width {
            let spaces = " ".repeat(width - total_chars);
            self.line(&format!("{}{}{}", left, spaces, right))
        } else {
            self.line(&format!("{} {}", left, right))
        }
    }

    /// Feed N lines
    pub fn feed(&mut self, lines: u8) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x64, lines]);
        self
    }

    /// GS V 66 0 - Partial or Full paper cut
    pub fn cut(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1D, 0x56, 0x42, 0x00]);
        self
    }

    /// Open cash drawer (kick pin 2)
    pub fn open_drawer(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x70, 0x00, 0x19, 0xFA]);
        self
    }

    /// Return raw bytes for sending to WebUSB, WebSerial or TCP socket
    pub fn build(self) -> Vec<u8> {
        self.buffer
    }

    /// Return base64 encoded string for JSON transport over WebSocket / SSE
    pub fn to_base64(&self) -> String {
        encode_base64(&self.buffer)
    }
}

const B64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_base64(bytes: &[u8]) -> String {
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };

        result.push(B64_CHARS[b0 >> 2] as char);
        result.push(B64_CHARS[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        if chunk.len() > 1 {
            result.push(B64_CHARS[((b1 & 0x0F) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(B64_CHARS[b2 & 0x3F] as char);
        } else {
            result.push('=');
        }
    }
    result
}
