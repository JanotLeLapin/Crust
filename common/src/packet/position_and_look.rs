pub struct PositionAndLookPacketBuilder {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: u8,
}

impl PositionAndLookPacketBuilder {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        }
    }

    pub fn yaw(mut self, yaw: f32) -> Self {
        self.yaw = yaw;
        self
    }

    pub fn pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch;
        self
    }

    pub fn finish(self, process_id: String) -> Vec<u8> {
        util::packet::PacketBuilder::new(0x08, process_id)
            .write_sized(self.x)
            .write_sized(self.y)
            .write_sized(self.z)
            .write_sized(self.yaw)
            .write_sized(self.pitch)
            .write_sized(self.flags)
            .finish()
    }
}

