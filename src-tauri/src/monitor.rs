use std::{thread, time::Duration};

use sysinfo::{System};
use tauri::Emitter;
use chrono::Local;
use crate::{
    rede::{interface_rede_atual, rede_rx_tx},
    ram::{ram_leitura},
    disco::{disco_os_data},
    modelos::{DadosCPU, DadosMonitor},
    idioma::{idioma}
};
use crate::MONITOR_SISTEMA;
use crate::modelos::Labels;
use crate::MONITOR;



//Verifica tempo do sistema:
pub(crate) fn tempo_sistema(labels: Labels) -> String {
    let segundos = System::uptime();
    let dias = segundos / 86_400;
    let horas = (segundos % 86_400) / 3_600;
    let minutos = (segundos % 3_600) / 60;

    if dias > 0 {
        labels.sobre.parte_sistema_tempo_sistema_label
          .replace("{dias}", &dias.to_string())
          .replace("{horas}", &horas.to_string())
        } else if horas > 0 {
            labels.sobre.parte_sistema_tempo_sistema_label2
              .replace("{horas}", &horas.to_string())
              .replace("{minutos}", &minutos.to_string())
            } else {
                labels.sobre.parte_sistema_tempo_sistema_label3
                  .replace("{minutos}", &minutos.to_string())
            }
}






pub(crate) fn monitorar_sistema(app: tauri::AppHandle) {
    
    let interface_atual = interface_rede_atual();


    loop {
        if !*MONITOR_SISTEMA.lock().unwrap() {
            thread::sleep(Duration::from_millis(200));
            continue;
        }

        thread::sleep(Duration::from_secs(1));
        let labels=idioma(app.clone());

        let mut monitor = MONITOR.get().unwrap().lock().unwrap();
        
        monitor.sistema.refresh_cpu_usage();
        monitor.sistema.refresh_memory();

        let total_ram = monitor.sistema.total_memory() as f64;
        let usada_ram = monitor.sistema.used_memory() as f64;

        monitor.discos.refresh(true);
        let disco = disco_os_data(&monitor.discos);

        
        let temperatura_cpu = monitor.componentes
            .iter()
            .find(|componente| componente.label().contains("Package id 0"))
            .and_then(|componente| componente.temperature())
            .map(|temperatura| temperatura.round() as u8);

        let uso_total_cpu = monitor.sistema.global_cpu_usage().round() as u8;
        //println!("Uso total da CPU: {uso_total_cpu}%");

        let mut modelo_cpu = String::new();
        let mut frequencia_cpu = String::new();
        if let Some(cpu) = monitor.sistema.cpus().first() {
            modelo_cpu = cpu.brand().to_string();
            frequencia_cpu = cpu.frequency().to_string();
            //println!("Modelo: {}", cpu.brand());
            //println!("Frequência: {} MHz", cpu.frequency());
        }

        //println!("--- REDE ---");
        let rede_atual = rede_rx_tx(&mut monitor.redes, interface_atual.clone());



        let dados_monitor = DadosMonitor {
            cpu: DadosCPU {
                modelo: modelo_cpu,
                frequencia: frequencia_cpu,
                uso_percentual: uso_total_cpu,
                temperatura: temperatura_cpu.unwrap_or(0),
            },
            ram: ram_leitura(total_ram, usada_ram),
            disco: disco,
            rede: rede_atual,
            tempo_os: tempo_sistema(labels.clone()),
            data_hora: Local::now().format(&labels.sobre.parte_sistema_data_hora_formato).to_string()
        };

        let _ = app.emit("DadosMonitor", dados_monitor);
    }
}

