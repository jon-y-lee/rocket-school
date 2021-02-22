pub struct Kalman {
    x: f32,
    q_angle: f32,
    q_bias: f32,
    r_measure: f32,
    angle: f32,
    bias: f32,
    rate: f32,
    // pMatrix: [[f32]]
}

// self.QAngle = 0.001
// self.QBias = 0.003
// self.RMeasure = 0.03
// self.angle = 0.0
// self.bias = 0.0
// self.rate = 0.0
// self.P=[[0.0,0.0],[0.0,0.0]]


impl Kalman {
    // pub fn setAngle(&mut self, angle: f32) -> Unit {
    //     println!("Setting Angle:{}", angle);
    //     self.qAngle = qAngle;
    // }
}
