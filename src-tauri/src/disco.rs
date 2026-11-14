use std::{ path::Path, thread, time::Duration};

use sysinfo::{Components, DiskKind, Disks, Networks, System};
use tauri::Emitter;

use crate::{
    modelos::{DadosDisco, ListaDisco, Disco},
};

const GB: f64 = 1_073_741_824.0;
const MB: f64 = 1_048_576.0;

// Identifica o tipo do disco.
fn tipo_disco(disco: &sysinfo::Disk) -> &'static str {
    let nome = disco.name().to_string_lossy().to_ascii_lowercase();

    if nome.contains("nvme") {
        return "NVMe";
    }

    match disco.kind() {
        DiskKind::HDD => "HD",
        DiskKind::SSD => "SSD",
        DiskKind::Unknown(_) => "Indefinido",
    }
}


pub(crate) fn disco_os_data()->DadosDisco {
    let mut discos = Disks::new_with_refreshed_list();
    discos.refresh(true);

    let mut dados_disco = DadosDisco {
        tipo: "Indefinido".to_string(),
        total: 0.0,
        usada: 0.0,
        restante: 0.0,
        uso_percentual: 0,
    };
    
    if let Some(disco) = discos.list().iter().find(|disco| disco.mount_point() == Path::new("/"))
        {
            let total = disco.total_space();
            let disponivel = disco.available_space();
            let usado = total.saturating_sub(disponivel);
            let tipo = tipo_disco(disco);
            let uso_percentual = if total > 0 {(usado as f64 / total as f64 * 100.0).round() as u8} else {0};

            dados_disco = DadosDisco {
                tipo: tipo.to_string(),
                total: (total as f64).round(),
                usada: (usado as f64).round(),
                restante: (disponivel as f64).round(),
                uso_percentual,
            };

        } 
        
    dados_disco


}

pub(crate) fn disco_lista() -> Vec<Disco> {
    let mut discos = Disks::new_with_refreshed_list();
    discos.refresh(true);
    let mut lista = Vec::new();
      for disco in discos.list() {//Percorre todos os discos

            let tipo = tipo_disco(disco);//HD, SSD, NVME
            let montagem = disco.mount_point().display().to_string();
            if tipo != "Indefinido" && montagem != "/boot/efi"{
                let total = disco.total_space();
                let disponivel = disco.available_space();
                let usado = total.saturating_sub(disponivel);
                lista.push(Disco {
                    dispositivo: disco.name().to_string_lossy().to_string(),
                    montagem: montagem,
                    tipo: tipo.to_string(),
                    total: (total as f64).round(),//Pega valor total
                    usada: (usado as f64).round(),//Pega valor usado
                    restante: (disponivel as f64).round(),//Pega valor disponível
                    uso_percentual: if total > 0 {(usado as f64 / total as f64 * 100.0).round() as u8} else {0},                
                });
            }

        }
        lista
    }