use std::{ path::Path, thread, time::Duration};

use sysinfo::{Components, DiskKind, Disks, Networks, System};
use tauri::Emitter;
use gfxinfo::{active_gpu, GpuInfo};

use crate::{
    modelos::{DadosGPU},
};
use crate::MONITOR_GPU;


const GB: f64 = 1_073_741_824.0;
const MB: f64 = 1_048_576.0;

pub(crate) fn monitorar_gpu(app: tauri::AppHandle){

    loop{
        if !*MONITOR_GPU.lock().unwrap() {
            continue;
        }
    thread::sleep(Duration::from_secs(1));

    let gpu = match active_gpu() {
        Ok(gpu) => gpu,
        Err(erro) => {
            eprintln!("Não foi possível obter a GPU: {erro}");
            return;
        }
    };

    let info = gpu.info();

    let mut dados_gpu = DadosGPU {
        fabricante: "Desconhecido".to_string(),
        modelo: "Desconhecido".to_string(),
        uso_percentual: 0,
        uso_percentual_memoria: 0,
        temperatura: 0.0,
        memoria_total: 0.0,
        memoria_usada: 0.0,
        memoria_restante: 0.0,
    };
    dados_gpu.fabricante=gpu.vendor().to_string();
    dados_gpu.modelo= gpu.model().to_string();

    dados_gpu.uso_percentual=info.load_pct() as u8;
    dados_gpu.temperatura=info.temperature() as f64 / 1000.0;

    dados_gpu.memoria_total=(info.total_vram() as f64).round();
    dados_gpu.memoria_usada=(info.used_vram() as f64).round();
    dados_gpu.memoria_restante=dados_gpu.memoria_total-dados_gpu.memoria_usada;
    dados_gpu.uso_percentual_memoria = 
    if dados_gpu.memoria_total > 0.0 {
        (dados_gpu.memoria_usada / dados_gpu.memoria_total * 100.0).round() as u8
    } else {0};

    let _ = app.emit("DadosMonitorGPU", dados_gpu);

}}