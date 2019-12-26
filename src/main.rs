use raylib::prelude::*;
use rand::prelude::*;
use arr_macro::arr;

extern crate lazy_static;

mod lights;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

struct Column {
    height: f32,
    position: Vector3,
    color: Color
}

impl Column {
    fn create_random() -> Column {
        let mut rng = rand::thread_rng();
        let height: f32 = rng.gen_range(1.0, 12.0);
        let position = Vector3::new(
            rng.gen_range(-15.0, 15.0),
            height / 2.0,
            rng.gen_range(-15.0, 15.0),
        );
        let color = Color::new(
            rng.gen_range(20, 255), 
            rng.gen_range(10, 55),
            30,
            255
        );
        
        Column {
            height,
            position,
            color
        }
    }

    fn generate_mesh(&self, thread: &RaylibThread) -> Mesh {
        Mesh::gen_mesh_cube(thread, 2.0, self.height, 2.0)
    }
}

fn render_scene<T: RaylibDraw3D>(d2: &mut T, models: &Vec<Model>, columns: &[Column; 20]) {
    d2.draw_plane(
        Vector3::new(0.0, 0.0, 0.0),
        Vector2::new(32.0, 32.0),
        Color::LIGHTGRAY
    );
    d2.draw_cube(
        Vector3::new(-16.0, 2.5, 0.0),
        1.0, 5.0, 32.0, Color::BLUE
    );
    d2.draw_cube(
        Vector3::new(16.0, 2.5, 0.0),
        1.0, 5.0, 32.0, Color::LIME
    );
    d2.draw_cube(
        Vector3::new(0.0, 2.5, 16.0),
        32.0, 5.0, 1.0, Color::GOLD
    );

    for (index, model) in models.into_iter().enumerate() {
        let position = columns[index].position;
        let color = columns[index].color;
        d2.draw_model(&model, position, 1.0, color);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 2.0, 4.0), 
        Vector3::new(0.0, 1.8, 0.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        60.0
    );
    let columns: [Column; 20] = arr![Column::create_random(); 20];
    let mut column_meshes: Vec<Mesh> = Vec::with_capacity(20);
    for column in columns.into_iter() {
        column_meshes.push(
            column.generate_mesh(&thread)
        )
    }
    let mut column_models: Vec<Model> = Vec::with_capacity(20);

    for (index, _) in columns.into_iter().enumerate() {
        let mesh = &column_meshes[index];
        column_models.push(
            rl.load_model_from_mesh(&thread, mesh).unwrap()
        )
    }
    // Borrow models vec to use in the loop
    let models = &column_models;

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FIRST_PERSON);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGREEN);
        {
            let mut d2 = d.begin_mode_3D(camera);
            render_scene(&mut d2, models, &columns);
        }
        d.draw_rectangle(10, 10, 220, 70, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 220, 70, Color::BLUE);
        d.draw_text("First person camera default controls:", 20, 20, 10, Color::BLACK);
        d.draw_text("- Move with keys: W, A, S, D", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Mouse move to look around", 40, 60, 10, Color::DARKGRAY);
    }
}

