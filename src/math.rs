fn mat3_mat3_mul(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        ],
    ]
}

pub fn mat4_mat4_mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0] + a[0][3] * b[3][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1] + a[0][3] * b[3][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2] + a[0][3] * b[3][2],
            a[0][0] * b[0][3] + a[0][1] * b[1][3] + a[0][2] * b[2][3] + a[0][3] * b[3][3],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0] + a[1][3] * b[3][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1] + a[1][3] * b[3][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2] + a[1][3] * b[3][2],
            a[1][0] * b[0][3] + a[1][1] * b[1][3] + a[1][2] * b[2][3] + a[1][3] * b[3][3],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0] + a[2][3] * b[3][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1] + a[2][3] * b[3][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2] + a[2][3] * b[3][2],
            a[2][0] * b[0][3] + a[2][1] * b[1][3] + a[2][2] * b[2][3] + a[2][3] * b[3][3],
        ],
        [
            a[3][0] * b[0][0] + a[3][1] * b[1][0] + a[3][2] * b[2][0] + a[3][3] * b[3][0],
            a[3][0] * b[0][1] + a[3][1] * b[1][1] + a[3][2] * b[2][1] + a[3][3] * b[3][1],
            a[3][0] * b[0][2] + a[3][1] * b[1][2] + a[3][2] * b[2][2] + a[3][3] * b[3][2],
            a[3][0] * b[0][3] + a[3][1] * b[1][3] + a[3][2] * b[2][3] + a[3][3] * b[3][3],
        ],
    ]
}

#[allow(unused)]
fn vec4_mat4_mul(vec4: [f32; 4], mat4: [[f32; 4]; 4]) -> [f32; 4] {
    [
        vec4[0] * mat4[0][0] + vec4[1] * mat4[1][0] + vec4[2] * mat4[2][0] + vec4[3] * mat4[3][0],
        vec4[0] * mat4[0][1] + vec4[1] * mat4[1][1] + vec4[2] * mat4[2][1] + vec4[3] * mat4[3][1],
        vec4[0] * mat4[0][2] + vec4[1] * mat4[1][2] + vec4[2] * mat4[2][2] + vec4[3] * mat4[3][2],
        vec4[0] * mat4[0][3] + vec4[1] * mat4[1][3] + vec4[2] * mat4[2][3] + vec4[3] * mat4[3][3],
    ]
}

fn vec3_mat3_mul(vec3: [f32; 3], mat3: [[f32; 3]; 3]) -> [f32; 3] {
    [
        vec3[0] * mat3[0][0] + vec3[1] * mat3[1][0] + vec3[2] * mat3[2][0],
        vec3[0] * mat3[0][1] + vec3[1] * mat3[1][1] + vec3[2] * mat3[2][1],
        vec3[0] * mat3[0][2] + vec3[1] * mat3[1][2] + vec3[2] * mat3[2][2],
    ]
}

pub fn rotate(vector: [f32; 3], alpha_yaw: f32, beta_pitch: f32, gamma_roll: f32) -> [f32; 3] {
    let r_alpha = [
        [alpha_yaw.cos(), -alpha_yaw.sin(), 0f32],
        [alpha_yaw.sin(), alpha_yaw.cos(), 0f32],
        [0f32, 0f32, 1f32],
    ];

    let r_beta = [
        [beta_pitch.cos(), 0f32, beta_pitch.sin()],
        [0f32, 1f32, 0f32],
        [-beta_pitch.sin(), 0f32, beta_pitch.cos()],
    ];

    let r_gamma = [
        [1f32, 0f32, 0f32],
        [0f32, gamma_roll.cos(), -gamma_roll.sin()],
        [0f32, gamma_roll.sin(), gamma_roll.cos()],
    ];

    let product = mat3_mat3_mul(mat3_mat3_mul(r_alpha, r_beta), r_gamma);

    let rot_mat = [
        [product[0][0], product[0][1], product[0][2]],
        [product[1][0], product[1][1], product[1][2]],
        [product[2][0], product[2][1], product[2][2]],
    ];

    vec3_mat3_mul(vector, rot_mat)
}

pub fn get_rotation_matrix(alpha_yaw: f32, beta_pitch: f32, gamma_roll: f32) -> [[f32; 4]; 4] {
    let r_alpha = [
        [alpha_yaw.cos(), -alpha_yaw.sin(), 0f32],
        [alpha_yaw.sin(), alpha_yaw.cos(), 0f32],
        [0f32, 0f32, 1f32],
    ];

    let r_beta = [
        [beta_pitch.cos(), 0f32, beta_pitch.sin()],
        [0f32, 1f32, 0f32],
        [-beta_pitch.sin(), 0f32, beta_pitch.cos()],
    ];

    let r_gamma = [
        [1f32, 0f32, 0f32],
        [0f32, gamma_roll.cos(), -gamma_roll.sin()],
        [0f32, gamma_roll.sin(), gamma_roll.cos()],
    ];

    let product = mat3_mat3_mul(mat3_mat3_mul(r_alpha, r_beta), r_gamma);

    [
        [product[0][0], product[0][1], product[0][2], 0.0],
        [product[1][0], product[1][1], product[1][2], 0.0],
        [product[2][0], product[2][1], product[2][2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn get_translation_matrix(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn get_scale_matrix(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
