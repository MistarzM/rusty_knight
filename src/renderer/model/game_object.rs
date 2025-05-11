pub struct Object {
    pub position: glm::Vec3,
    pub angle: f32,
}

pub struct Camera {
    pub position: glm::Vec3,
    pub forwards: glm::Vec3,
    pub right: glm::Vec3,
    pub up: glm::Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        let position = glm::Vec3::new(-5.0, 0.0, 2.0);
        // horizontal
        let yaw: f32 = 0.0;
        // vertical
        let pitch: f32 = 0.0;

        let forwards = glm::Vec3::new(1.0, 0.0, 0.0);
        let right = glm::Vec3::new(0.0, -1.0, 0.0);
        let up = glm::Vec3::new(0.0, 0.0, 1.0);

        Camera {
            position,
            forwards,
            right,
            up,
            yaw,
            pitch,
        }
    }

    pub fn camera_spin(&mut self, d_yaw: f32, d_pitch: f32) {
        self.yaw += d_yaw;
        if self.yaw > 360.0 {
            self.yaw -= 360.0;
        }
        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        self.pitch = glm::min(89.0, glm::max(-89.0, self.pitch + d_pitch));

        let c = glm::cos(glm::radians(self.yaw));
        let s = glm::sin(glm::radians(self.yaw));
        let c2 = glm::cos(glm::radians(self.pitch));
        let s2 = glm::sin(glm::radians(self.pitch));

        self.forwards.x = c * c2;
        self.forwards.y = s * c2;
        self.forwards.z = s2;

        self.up.x = 0.0;
        self.up.y = 0.0;
        self.up.z = 1.0;

        self.right = glm::normalize(glm::cross(self.forwards, self.up));
        self.up = glm::normalize(glm::cross(self.right, self.forwards));
    }

    pub fn camera_move(&mut self, d_right: f32, d_forwards: f32) {
        let z: f32 = self.position.z;
        self.position = self.position + self.right * d_right + self.forwards * d_forwards;
        self.position.z = z;
    }
}
