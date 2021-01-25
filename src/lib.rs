use cgmath::{vec3, Vector3, Point3, InnerSpace, Matrix4};

pub struct Camera
{
    position_: Point3<f32>,
    front_: Vector3<f32>,
    up_: Vector3<f32>,
    right_: Vector3<f32>,
    world_up_: Vector3<f32>,
    yaw_: f32,
    pitch_: f32,
    speed_: f32,
}

impl Camera{
    pub fn new(position: Point3<f32>, up: Vector3<f32>, yaw: f32, pitch: f32, speed: f32) -> Camera
    {

        let mut camera = Camera{
            position_: position,
            front_: vec3(0.0, 0.0, 0.0),
            up_: vec3(0.0, 0.0, 0.0),
            right_: vec3(0.0, 0.0, 0.0),
            world_up_: up,
            yaw_: yaw,
            pitch_: pitch,
            speed_: speed,
        };

        camera.update();

        camera
    }

    pub fn update(& mut self)
    {
        self.front_ = vec3(self.yaw_.to_radians().cos() * self.pitch_.to_radians().cos(),
                         self.pitch_.to_radians().sin(),
                         self.yaw_.to_radians().sin() * self.pitch_.to_radians().cos()).normalize();

        self.right_ = self.front_.cross(self.world_up_).normalize();
        self.up_ = self.right_.cross(self.front_).normalize();
    }

    pub fn move_forward(&mut self, dt: f32)
    {
        self.position_ += self.front_ * self.speed_ * dt;
    }

    pub fn move_backward(&mut self, dt: f32)
    {
        self.position_ -= self.front_ * self.speed_ * dt;
    }

    pub fn move_right(&mut self, dt: f32)
    {
        self.position_ += self.right_ * self.speed_ * dt;
    }

    pub fn move_left(&mut self, dt: f32)
    {
        self.position_ -= self.right_ * self.speed_ * dt;
    }

    pub fn view(&self) -> Matrix4<f32>
    {
        Matrix4::look_at_rh(self.position_, self.position_ + self.front_, self.up_)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
