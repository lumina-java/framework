/// Returns client-side JavaScript / WebAssembly bridge code that runs in the browser
pub fn device_bridge_script() -> &'static str {
    r#"
/**
 * Lumina Device Bridge (WebUSB, WebSerial, WebHID, WebRTC)
 * Connects Linux Cloud Server to Client Hardware
 */
window.LuminaDevice = {
    // ── Thermal Receipt Printer (WebUSB / WebSerial) ──
    printer: {
        device: null,
        async connectUsb(vendorId = 0x04b8) {
            try {
                this.device = await navigator.usb.requestDevice({ filters: [{ vendorId }] });
                await this.device.open();
                await this.device.selectConfiguration(1);
                await this.device.claimInterface(0);
                console.log("✅ [LuminaDevice] USB Printer connected:", this.device.productName);
                return true;
            } catch (err) {
                console.error("❌ [LuminaDevice] Failed to connect USB printer:", err);
                return false;
            }
        },
        async printBase64(dataBase64) {
            if (!this.device) {
                console.warn("⚠️ [LuminaDevice] Printer not connected. Requesting connection...");
                const connected = await this.connectUsb();
                if (!connected) return false;
            }
            try {
                const binary = atob(dataBase64);
                const bytes = new Uint8Array(binary.length);
                for (let i = 0; i < binary.length; i++) {
                    bytes[i] = binary.charCodeAt(i);
                }
                await this.device.transferOut(1, bytes);
                console.log("✅ [LuminaDevice] Print completed successfully.");
                return true;
            } catch (err) {
                console.error("❌ [LuminaDevice] Print failed:", err);
                return false;
            }
        }
    },

    // ── Barcode & Serial Scanner (WebSerial) ──
    scanner: {
        port: null,
        async listen(baudRate = 9600, onBarcode) {
            try {
                this.port = await navigator.serial.requestPort();
                await this.port.open({ baudRate });
                console.log("✅ [LuminaDevice] Serial Scanner connected.");

                const textDecoder = new TextDecoderStream();
                this.port.readable.pipeTo(textDecoder.writable);
                const reader = textDecoder.readable.getReader();

                let buffer = "";
                while (true) {
                    const { value, done } = await reader.read();
                    if (done) break;
                    if (value) {
                        buffer += value;
                        if (buffer.includes("\n") || buffer.includes("\r")) {
                            const code = buffer.trim();
                            buffer = "";
                            if (code.length > 0 && onBarcode) {
                                onBarcode(code);
                            }
                        }
                    }
                }
            } catch (err) {
                console.error("❌ [LuminaDevice] Scanner error:", err);
            }
        }
    },

    // ── WebSocket Real-time Hardware Listener ──
    connectServerBridge(wsUrl) {
        const socket = new WebSocket(wsUrl);
        socket.onmessage = async (event) => {
            try {
                const msg = JSON.parse(event.data);
                if (msg.type === "PrintRaw") {
                    const ok = await this.printer.printBase64(msg.payload.data_base64);
                    socket.send(JSON.stringify({
                        status: ok ? "Success" : "Error",
                        result: { device: "printer", message: ok ? "Printed" : "Print failed" }
                    }));
                }
            } catch (e) {
                console.error("Bridge dispatch error:", e);
            }
        };
        console.log("🔗 [LuminaDevice] Hardware WebSocket Bridge connected:", wsUrl);
    }
};
"#
}
