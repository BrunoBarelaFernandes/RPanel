
use std::sync::{Mutex};
use sysinfo::{Components, Disks, Networks, System};
use std::thread;
use tauri::Manager;
use std::sync::{OnceLock, LazyLock};
mod modelos;
mod monitor;
mod dados_sobre;
mod rede;
mod ram;
mod disco;
mod gpu;
pub mod ModulosProcessos;
mod processos;
use crate::modelos::{DadosSysInfo, InfoRPanel};
mod idioma;
mod monitor_tv;
mod relatorio;
mod info_rpanel;
use crate::info_rpanel::INFO_RPANEL;
pub static MONITOR: OnceLock<Mutex<DadosSysInfo>> = OnceLock::new();



pub(crate) static MONITOR_SISTEMA: Mutex<bool> = Mutex::new(false);
pub(crate) static MONITOR_GPU: Mutex<bool> = Mutex::new(false);
pub(crate) static MONITOR_PROCESSOS: Mutex<bool> = Mutex::new(false);



#[tauri::command]
fn solicitar(sistema: String, gpu: String, processos: String){
    *MONITOR_SISTEMA.lock().unwrap() = sistema == "true";
    *MONITOR_GPU.lock().unwrap() = gpu == "true";
    *MONITOR_PROCESSOS.lock().unwrap() = processos == "true";
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if MONITOR.set(Mutex::new(DadosSysInfo {
                sistema: System::new_all(),
                componentes: Components::new_with_refreshed_list(),
                discos: Disks::new_with_refreshed_list(),
                redes: Networks::new_with_refreshed_list(),
            })).is_err() {
                return Err("Falha ao inicializar monitor".into());
            }
    
            let app_monitor = app.handle().clone();
            let app_internet = app.handle().clone();
            let app_monitor_gpu = app.handle().clone();
            let app_monitor_processos = app.handle().clone();

            thread::spawn(move || monitor::monitorar_sistema(app_monitor));
            thread::spawn(move || rede::indicador_internet(app_internet));
            thread::spawn(move || gpu::monitorar_gpu(app_monitor_gpu));
            thread::spawn(move || processos::processos(app_monitor_processos));
            let janela = app.get_webview_window("main")
            .expect("Janela principal não encontrada");
            janela.maximize()?;


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            solicitar,
            ram::ram_dados,
            dados_sobre::info,
            processos::encerrar_app,
            idioma::idioma,
            idioma::mudar_idioma,
            idioma::lista_idiomas,
            relatorio::criar_relatorio,
            ])
        .run(tauri::generate_context!())
        .expect("Erro aplicação RPanel");
}
