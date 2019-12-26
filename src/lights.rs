mod lights {
    use std::sync::atomic;
    use std::os::raw::c_void;
    use raylib::prelude::*;

    const MAX_LIGHTS: usize = 4;

    #[derive(Clone, Copy)]
    pub enum LightType {
        DIRECTIONAL_LIGHT,
        POINT_LIGHT
    }

    impl ShaderV for LightType {
        const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::UNIFORM_INT;
        unsafe fn value(&self) -> *const c_void {
            match self {
                LightType::DIRECTIONAL_LIGHT => 0 as *const i32 as *const c_void,
                LightType::POINT_LIGHT => 1 as *const i32 as *const c_void
            }
        }
    }

    pub struct Light {
        light_type: LightType,
        position: Vector3,
        target: Vector3,
        color: Color,
        is_enabled: i32,

        // Shader locations
        enabled_loc: i32,
        type_loc: i32,
        position_loc: i32,
        target_loc: i32,
        color_loc: i32
    }

    // Keep track of the number of lights
    static LIGHTS_AMOUNT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    impl Light {

        fn get_lights_amount() -> usize {
            LIGHTS_AMOUNT.load(atomic::Ordering::SeqCst)
        }

        fn increment_lights_amount() {
            LIGHTS_AMOUNT.fetch_add(1, atomic::Ordering::SeqCst);
        }

        pub fn new (
            light_type: LightType,
            position: Vector3,
            target: Vector3,
            color: Color,
            is_enabled: bool,
            shader:&mut Shader) -> Option<Light> {
                // Don't make more lights if we've at the limit
                if Light::get_lights_amount() == MAX_LIGHTS {
                    return None
                };

                let light = Light {
                    light_type, position, color, target,
                    is_enabled: match is_enabled {
                        true => 1,
                        false => 0
                    },
                    enabled_loc: shader.get_shader_location("lights[x].enabled\0"),
                    type_loc: shader.get_shader_location("lights[x].type\0"),
                    position_loc: shader.get_shader_location("lights[x].position\0"),
                    target_loc: shader.get_shader_location("lights[x].target\0"),
                    color_loc: shader.get_shader_location("lights[x].color\0")
                };

                light.update_light_values(shader);
                Light::increment_lights_amount();

                Some(light)
        }

        fn update_light_values(&self, shader: &mut Shader) {
            // Send to shader light enabled state and type
            shader.set_shader_value(self.enabled_loc, self.is_enabled);
            shader.set_shader_value(self.type_loc, self.light_type);

            // Sent to shader light position values
            let position: [f32; 3] = [self.position.x, self.position.y, self.position.z];
            shader.set_shader_value(self.position_loc, position);

            // Send to shader light target position values
            let target_position: [f32; 3] = [self.target.x, self.target.y, self.target.z];
            shader.set_shader_value(self.target_loc, target_position);

            // Send to shader light color values
            let color: [f32; 4] = [
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
                self.color.a as f32 / 255.0
            ];
            shader.set_shader_value(self.color_loc, color);
        }
    }
}