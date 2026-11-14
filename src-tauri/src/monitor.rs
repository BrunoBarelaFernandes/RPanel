use std::{ path::Path, thread, time::Duration};

use sysinfo::{Components, DiskKind, Disks, Networks, System};
use tauri::Emitter;
use chrono::Local;
use crate::{
    rede::{interface_rede_atual, rede_rx_tx},
    ram::{ram_dados, ram_leitura},
    disco::{disco_os_data},
    modelos::{DadosCPU, DadosDisco, DadosMonitor, DadosRam, DadosRede, DadosGPU},
};
use crate::MONITOR_SISTEMA;

const GB: f64 = 1_073_741_824.0;
const MB: f64 = 1_048_576.0;


//Verifica tempo do sistema:
fn tempo_sistema() -> String {
    let segundos = System::uptime();
    let dias = segundos / 86_400;
    let horas = (segundos % 86_400) / 3_600;
    let minutos = (segundos % 3_600) / 60;

    if dias > 0 {
        format!("{dias} dias, {horas} horas")
    } else if horas > 0 {
        format!("{horas} horas e {minutos} minutos")
    } else {
        format!("{minutos} minutos")
    }
}






pub(crate) fn monitorar_sistema(app: tauri::AppHandle) {
    let mut sistema = System::new_all();
    let mut discos = Disks::new_with_refreshed_list();
    let mut redes = Networks::new_with_refreshed_list();
    let mut componentes = Components::new_with_refreshed_list();
    let interface_atual = interface_rede_atual();

    // Primeira leitura da CPU e da rede.
    sistema.refresh_cpu_usage();
    redes.refresh(true);

    loop {
        if !*MONITOR_SISTEMA.lock().unwrap() {
            continue;
        }
        
        thread::sleep(Duration::from_secs(1));

        sistema.refresh_cpu_usage();
        sistema.refresh_memory();
        discos.refresh(true);
        redes.refresh(true);
        componentes.refresh(true);
        
        let temperatura_cpu = componentes
            .iter()
            .find(|componente| componente.label().contains("Package id 0"))
            .and_then(|componente| componente.temperature())
            .map(|temperatura| temperatura.round() as u8);

        let uso_total_cpu = sistema.global_cpu_usage().round() as u8;
        //println!("Uso total da CPU: {uso_total_cpu}%");

        let mut modelo_cpu = String::new();
        let mut frequencia_cpu = String::new();
        if let Some(cpu) = sistema.cpus().first() {
            modelo_cpu = cpu.brand().to_string();
            frequencia_cpu = cpu.frequency().to_string();
            //println!("Modelo: {}", cpu.brand());
            //println!("Frequência: {} MHz", cpu.frequency());
        }

        //println!("--- REDE ---");
        let interface_rede = interface_atual.clone();
        let mut rede_atual = rede_rx_tx(interface_rede);



        let dados_monitor = DadosMonitor {
            cpu: DadosCPU {
                modelo: modelo_cpu,
                frequencia: frequencia_cpu,
                uso_percentual: uso_total_cpu,
                temperatura: temperatura_cpu.unwrap_or(0),
            },
            ram: ram_leitura(),
            disco: disco_os_data(),
            rede: rede_atual,
            tempo_os: tempo_sistema(),
            data_hora: Local::now().format("%d/%m/%Y - %H:%M").to_string()
        };

        let _ = app.emit("DadosMonitor", dados_monitor);
    }
}

