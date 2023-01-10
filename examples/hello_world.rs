use reindeer::backend::Backend;
use reindeer::common::{Shader, Vec3};
use reindeer::entity::Entity;
use reindeer::math::{
    get_rotation_matrix, get_scale_matrix, get_translation_matrix, mat4_mat4_mul,
};
use reindeer::scene::Drawable;
use reindeer::{self, ShaderProgram, VERTEX_ATTRIBUTE_FVEC3};

#[cfg_attr(feature = "webgl", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn run() {
    reindeer::init();

    let light_position = [0.5, 0.5, -0.8];
    let light_color = [0.5, 0.5, 1.0];
    let object_color = [1.0, 0.5, 0.5];

    let shader_program = reindeer::ShaderProgram::new("phong_light_object");
    shader_program.set_uniform("u_color", object_color);
    shader_program.set_uniform("u_light_color", light_color);
    shader_program.set_uniform("u_light_position", light_position);
    shader_program.set_uniform("u_camera_position", [0.0, 0.0, -1.0]);

    let shader_program_obj2 = ShaderProgram::new("uniform_color");
    shader_program_obj2.set_uniform("u_color", light_color);

    let vertices: Vec<Vec3> = vec![
        [-0.5, -0.5, -0.5],
        [0.0, 0.0, -1.0],
        [0.5, -0.5, -0.5],
        [0.0, 0.0, -1.0],
        [0.5, 0.5, -0.5],
        [0.0, 0.0, -1.0],
        [0.5, 0.5, -0.5],
        [0.0, 0.0, -1.0],
        [-0.5, 0.5, -0.5],
        [0.0, 0.0, -1.0],
        [-0.5, -0.5, -0.5],
        [0.0, 0.0, -1.0],
        [-0.5, -0.5, 0.5],
        [0.0, 0.0, 1.0],
        [0.5, -0.5, 0.5],
        [0.0, 0.0, 1.0],
        [0.5, 0.5, 0.5],
        [0.0, 0.0, 1.0],
        [0.5, 0.5, 0.5],
        [0.0, 0.0, 1.0],
        [-0.5, 0.5, 0.5],
        [0.0, 0.0, 1.0],
        [-0.5, -0.5, 0.5],
        [0.0, 0.0, 1.0],
        [-0.5, 0.5, 0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, 0.5, -0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, -0.5, -0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, -0.5, -0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, -0.5, 0.5],
        [-1.0, 0.0, 0.0],
        [-0.5, 0.5, 0.5],
        [-1.0, 0.0, 0.0],
        [0.5, 0.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.5, 0.5, -0.5],
        [1.0, 0.0, 0.0],
        [0.5, -0.5, -0.5],
        [1.0, 0.0, 0.0],
        [0.5, -0.5, -0.5],
        [1.0, 0.0, 0.0],
        [0.5, -0.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.5, 0.5, 0.5],
        [1.0, 0.0, 0.0],
        [-0.5, -0.5, -0.5],
        [0.0, -1.0, 0.0],
        [0.5, -0.5, -0.5],
        [0.0, -1.0, 0.0],
        [0.5, -0.5, 0.5],
        [0.0, -1.0, 0.0],
        [0.5, -0.5, 0.5],
        [0.0, -1.0, 0.0],
        [-0.5, -0.5, 0.5],
        [0.0, -1.0, 0.0],
        [-0.5, -0.5, -0.5],
        [0.0, -1.0, 0.0],
        [-0.5, 0.5, -0.5],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, -0.5],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, 0.5],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, 0.5],
        [0.0, 1.0, 0.0],
        [-0.5, 0.5, 0.5],
        [0.0, 1.0, 0.0],
        [-0.5, 0.5, -0.5],
        [0.0, 1.0, 0.0],
    ];

    let obj = Entity::new(
        vertices.clone(),
        Some(shader_program),
        Some(vec![
            Some(VERTEX_ATTRIBUTE_FVEC3),
            None,
            Some(VERTEX_ATTRIBUTE_FVEC3),
        ]),
    );
    let light_source = Entity::new(
        vertices,
        Some(shader_program_obj2),
        Some(vec![
            Some(VERTEX_ATTRIBUTE_FVEC3),
            None,
            Some(VERTEX_ATTRIBUTE_FVEC3),
        ]),
    );

    let scene = vec![obj, light_source];

    reindeer::set_clear_color(1.0, 1.0, 1.0, 1.0);

    let rotation_angle = 0.002;
    let mut rotation_amount = 0.0;
    let mut light_source_rotation_amount = 42.42;

    let translation =
        get_translation_matrix(light_position[0], light_position[1], light_position[2]);
    let scale = get_scale_matrix(0.05, 0.05, 0.05);
    let light_source_model = mat4_mat4_mul(translation, scale);

    reindeer::lib::Context::draw_loop(move || {
        // BACKEND.lock().unwrap().before_draw();

        let rotation = get_rotation_matrix(rotation_amount, rotation_amount, rotation_amount);
        let light_source_rotation = get_rotation_matrix(
            light_source_rotation_amount,
            light_source_rotation_amount,
            light_source_rotation_amount,
        );

        scene.draw();

        scene[0]
            .shader
            .as_ref()
            .unwrap()
            .set_uniform("u_model", rotation);

        scene[1].shader.as_ref().unwrap().set_uniform(
            "u_model",
            mat4_mat4_mul(light_source_model, light_source_rotation),
        );
        rotation_amount += rotation_angle;
        light_source_rotation_amount += rotation_angle;

        // BACKEND.lock().unwrap().after_draw();
    });
}
fn main() {
    run();
}
