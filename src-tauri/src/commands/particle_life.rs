use crate::simulation::SimulationManager;
use crate::simulations::traits::SimulationType;
use bytemuck;
use std::sync::Arc;
use tauri::State;
use wgpu::util::DeviceExt;

#[tauri::command]
pub async fn scale_force_matrix(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
    scale_factor: f32,
) -> Result<String, String> {
    tracing::debug!(
        "scale_force_matrix called with scale_factor: {}",
        scale_factor
    );
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.scale_force_matrix(scale_factor);

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix scaled successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn flip_force_matrix_horizontal(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("flip_force_matrix_horizontal called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.flip_horizontal();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix flipped horizontally successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn flip_force_matrix_vertical(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("flip_force_matrix_vertical called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.flip_vertical();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix flipped vertically successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn rotate_force_matrix_clockwise(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("rotate_force_matrix_clockwise called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.rotate_clockwise();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix rotated clockwise successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn rotate_force_matrix_counterclockwise(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("rotate_force_matrix_counterclockwise called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.rotate_counterclockwise();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix rotated counterclockwise successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn shift_force_matrix_left(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("shift_force_matrix_left called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.shift_left();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix shifted left successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn shift_force_matrix_right(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("shift_force_matrix_right called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.shift_right();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix shifted right successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn shift_force_matrix_up(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("shift_force_matrix_up called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.shift_up();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix shifted up successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn shift_force_matrix_down(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("shift_force_matrix_down called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.shift_down();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix shifted down successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn zero_force_matrix(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("zero_force_matrix called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.zero_matrix();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix zeroed successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}

#[tauri::command]
pub async fn flip_force_matrix_sign(
    manager: State<'_, Arc<tokio::sync::Mutex<SimulationManager>>>,
    gpu_context: State<'_, Arc<tokio::sync::Mutex<crate::GpuContext>>>,
) -> Result<String, String> {
    tracing::debug!("flip_force_matrix_sign called");
    let mut sim_manager = manager.lock().await;
    let gpu_ctx = gpu_context.lock().await;

    if let Some(SimulationType::ParticleLife(simulation)) = &mut sim_manager.current_simulation {
        simulation.settings.flip_sign();

        // Update the force matrix buffer on GPU
        let force_matrix_data =
            crate::simulations::particle_life::simulation::ParticleLifeModel::flatten_force_matrix(
                &simulation.settings.force_matrix,
            );
        simulation.force_matrix_buffer =
            gpu_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Force Matrix Buffer"),
                    contents: bytemuck::cast_slice(&force_matrix_data),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });

        // Recreate bind groups that use this buffer
        simulation.recreate_bind_groups_with_force_matrix(&gpu_ctx.device);

        Ok("Force matrix sign flipped successfully".to_string())
    } else {
        Err("This command is only available for Particle Life simulation".to_string())
    }
}
