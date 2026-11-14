use std::{ path::Path, thread, time::Duration};

use sysinfo::{Components, DiskKind, Disks, Networks, System};

use crate::rede::{interface_rede_atual, rede_mac, rede_ip};
use crate::disco::{disco_lista};
use std::fs;
use std::env;
use crate::{
    modelos::{DadosSobre, ListaDisco},
};

const GB: f64 = 1_073_741_824.0;
const MB: f64 = 1_048_576.0;

//Verifica idioma local sistema linux
fn idioma_sistema() -> String {
    let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|variavel| {
            std::env::var(variavel)
                .ok()
                .filter(|valor| !valor.trim().is_empty())
        })
        .unwrap_or_else(|| "C".to_string());

    // Ex.: "pt_BR.UTF-8" vira "pt_BR".
    locale
        .split(|caractere| caractere == '.' || caractere == '@')
        .next()
        .unwrap_or("C")
        .to_string()
}

//Ler arquivo DMI:
fn ler_dmi(campo: &str) -> String {
    fs::read_to_string(format!("/sys/class/dmi/id/{campo}"))
        .map(|valor| valor.trim().to_string())
        .ok()
        .filter(|valor| !valor.is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn sistema_janelas() -> String {
    match env::var("XDG_SESSION_TYPE").as_deref() {
        Ok("wayland") => "Wayland".to_string(),
        Ok("x11") => "X11".to_string(),
        Ok(tipo) => tipo.to_string(),
        Err(_) => {
            if env::var_os("WAYLAND_DISPLAY").is_some() {
                "Wayland".to_string()
            } else if env::var_os("DISPLAY").is_some() {
                "X11".to_string()
            } else {
                "-".to_string()
            }
        }
    }
}



#[tauri::command]
pub(crate) fn info() -> DadosSobre{
    let mut dados = DadosSobre {
        nome_os: "Indefinido".to_string(),
        versao_os: "Indefinido".to_string(),
        kernel: "Indefinido".to_string(),
        sistema_janelas: "Indefinido".to_string(),
        nome_host: "Indefinido".to_string(),
        idioma: "Indefinido".to_string(),
        interface_rede: "Indefinido".to_string(),
        mac: "Indefinido".to_string(),
        ip: "Indefinido".to_string(),
        fabricante_placa_mae: "Indefinido".to_string(),
        modelo_placa_mae: "Indefinido".to_string(),
        versao_firmware: "Indefinido".to_string(),
        discos: ListaDisco {lista: Vec::new(),},
    };
    

    dados.nome_os = System::name().unwrap_or_else(|| "Desconhecido".to_string());
    dados.versao_os = System::os_version().unwrap_or_else(|| "Desconhecido".to_string());
    dados.kernel = System::kernel_version().unwrap_or_else(|| "Desconhecido".to_string());
    dados.sistema_janelas=sistema_janelas();
    dados.nome_host = System::host_name().unwrap_or_else(|| "Desconhecido".to_string());
    dados.idioma = idioma_sistema();
    dados.interface_rede = interface_rede_atual().expect("REASON");
    dados.mac = rede_mac(interface_rede_atual());
    dados.ip = rede_ip(interface_rede_atual());
    dados.fabricante_placa_mae=ler_dmi("board_vendor");
    dados.modelo_placa_mae=ler_dmi("board_name");
    dados.versao_firmware=ler_dmi("bios_version");
    dados.discos.lista=disco_lista();

    dados

}