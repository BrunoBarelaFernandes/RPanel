
use sysinfo::{Disks, DiskKind, Networks, System, Components, ProcessesToUpdate, ProcessRefreshKind};
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex, mpsc};
use std::{process::Command, sync::OnceLock};
use gfxinfo::{active_gpu, GpuInfo};
use std::fs;
use std::env;
use chrono::format::parse;
use std::path::PathBuf;
use std::path::Path;
use icon_finder::find_icon;
use crate::{ModulosProcessos::{app, flatpak, snap, steam}, modelos::DadosProcesso};
use tauri::Emitter;
use std::collections::HashMap;
use crate::MONITOR_PROCESSOS;


fn corretor_nome(nome: &str)->String{
let nomes: HashMap<String, String> = HashMap::from([
    ("oosplash".to_string(), "LibreOffice".to_string()),
]);

if let Some(n) = nomes.get(nome){n.to_string()}
else{nome.to_string()}

}

pub(crate) fn encontrar_desktop(nome: &str) -> Option<PathBuf> {
    let home = std::env::var("HOME").unwrap();
    let diretorios = [
        "/usr/share/applications".to_string(),
        format!("{}/.local/share/applications", home),
        "/var/lib/flatpak/exports/share/applications".to_string(),
        format!("{}/.local/share/flatpak/exports/share/applications", home),
        format!("{}/.local/share/flatpak/app/", home),
        "/var/lib/flatpak/app/".to_string(),
        "/var/lib/snapd/desktop/applications".to_string(),
    ];

    for diretorio in diretorios {
        let Ok(arquivos) = fs::read_dir(diretorio) else {continue;};

        for arquivo in arquivos.flatten() {
            let caminho = arquivo.path();

            if caminho.extension().and_then(|v| v.to_str()) != Some("desktop") {continue;}

            let arquivo_nome = caminho
                .file_stem()
                .and_then(|v| v.to_str())
                .unwrap_or("");

            if arquivo_nome.to_lowercase().contains(&nome.to_lowercase()) {
                return Some(caminho);
            }
        }
    }

    None
}




fn extrair_nome_icone(caminho: &Path) -> Option<String> {
    let conteudo = fs::read_to_string(caminho).ok()?;

    for linha in conteudo.lines() {
        if let Some(icone) = linha.strip_prefix("Icon=") {
            return Some(icone.trim().to_string());
        }
    }

    None
}


pub(crate) fn resolver_icone(nome: String)->String{
    let mut valor_icone="".to_string();
    if let Some(caminho) = encontrar_desktop(&nome) {
        if let Some(icone) = extrair_nome_icone(&caminho) {
            let ico = find_icon(&icone, 128);
            match ico {
                Some(caminho) => valor_icone = std::fs::canonicalize(&caminho.display().to_string())
                .unwrap_or(caminho.display().to_string().into())
                .to_string_lossy()
                .into_owned(),
                None => valor_icone = "".to_string(),
            }
        }

    }else{
        let ico = find_icon(&nome, 128);
        match ico {
            Some(caminho) => valor_icone = std::fs::canonicalize(&caminho.display().to_string())
                .unwrap_or(caminho.display().to_string().into())
                .to_string_lossy()
                .into_owned(),
            None => {
                let ico2 = nome.split('-').next()
                .and_then(|nome| find_icon(nome, 128));
                
                match ico2 {
                    Some(caminho) => valor_icone = std::fs::canonicalize(&caminho.display().to_string())
                .unwrap_or(caminho.display().to_string().into())
                .to_string_lossy()
                .into_owned(),
                    None => valor_icone = "".to_string(),
                }
            },
        }
    }
    valor_icone
}


pub(crate) fn pids_da_arvore(pid_principal: u32) -> Vec<u32> {
    let pid_argumento = pid_principal.to_string();
    let saida = Command::new("pstree")
        .args(["-T", "-p", "-l"])
        .arg(pid_argumento)
        .output();

    let Ok(saida) = saida else {
        return vec![pid_principal];
    };
    if !saida.status.success() {
        return vec![pid_principal];
    }

    let texto = String::from_utf8_lossy(&saida.stdout);
    let mut pids = Vec::new();
    for trecho in texto.split('(').skip(1) {
        if let Some((numero, _)) = trecho.split_once(')') {
            if let Ok(pid) = numero.parse::<u32>() {
                pids.push(pid);
            }
        }
    }

    pids.push(pid_principal);
    pids.sort_unstable();
    pids.dedup();
    pids
}



fn filtro_pids() -> (Vec<u32>, Vec<u32>) {
    let saida = Command::new("systemctl")
        .args([
            "--user",
            "list-units",
            "--state=running",
            "--no-legend",
            "--plain",
        ])
        .output()
        .unwrap();

    let texto = String::from_utf8(saida.stdout).unwrap();
    let mut pids = Vec::new();
    let mut pids_jogos_steam = Vec::new();

    for linha in texto.lines() {
        let Some(nome) = linha.split_whitespace().next() else {
            continue;
        };



        if nome.starts_with("app-flatpak-") && nome.ends_with(".scope") {
            let pid = flatpak::pid(nome);
            if pid != 0 {
                pids.push(pid);
            }
        } else if nome.starts_with("app-") {
            let pid = app::pid(nome);
            if pid != 0 {
                pids.push(pid);

                if nome.to_lowercase().contains("steam") {
                    if let Some(pid_jogo) = steam::pid(pid) {
                        pids.push(pid_jogo);
                        pids_jogos_steam.push(pid_jogo);
                    }
                }
            }
        } else if nome.starts_with("snap.") && nome.ends_with(".scope") {
            let pid = snap::pid(nome);
            if pid != 0 {
                pids.push(pid);
            }
        }
    }
    
    pids.extend(app::pid_desktop());
    pids.sort_unstable();
    pids.dedup();
    (pids, pids_jogos_steam)
    
}






fn maiuscula_inicial(texto: &str) -> String {
    let mut caracteres = texto.chars();

    match caracteres.next() {
        Some(primeiro) => primeiro.to_uppercase().chain(caracteres).collect(),
        None => String::new(),
    }
}




#[tauri::command]
pub(crate) fn encerrar_app(pid: u32) -> Result<(), String> {
    if pid <= 1 {
        return Err("PID inválido.".to_string());
    }

    let saida = Command::new("/usr/bin/kill")
        .arg(pid.to_string())
        .output()
        .map_err(|erro| format!("Não foi possível executar kill: {erro}"))?;

    if saida.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&saida.stderr).trim().to_string())
    }
}





#[tauri::command]
pub(crate) fn processos(app: tauri::AppHandle){
    let mut jogo_steam = 0;
    let mut sistema = System::new_all();
    let quantidade_cpus = sistema.cpus().len().max(1) as f64;
    let mut quantidade = sistema.processes().values().count();
    let pids = Arc::new(Mutex::new(filtro_pids()));
    let pids_thread = Arc::clone(&pids);
    let (tx, rx) = mpsc::channel();
    let mut start=0;
            let thread_att_pid = thread::spawn(move || {
                while let Ok(mensagem) = rx.recv() {
                    if mensagem == "start" && start == 0{
                        //println!("---------------------------------------------------------------");
                        start=1;
                        thread::sleep(Duration::from_secs(5));
                        *pids_thread.lock().unwrap() = filtro_pids();
                        start=0;
                    }
                }
            });

    loop{
        if !*MONITOR_PROCESSOS.lock().unwrap() {
            continue;
        }
        thread::sleep(Duration::from_secs(1));

        
        sistema.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::everything().without_tasks(),
        );
        let quantidade_atual = sistema.processes().len();
        if quantidade != quantidade_atual {
            quantidade = quantidade_atual;
            tx.send("start").unwrap();
        }

        let (mut pids_principais, pids_jogos_steam) =pids.lock().unwrap().clone();
        pids_principais.sort_unstable();
        pids_principais.dedup();
        let mut lista: Vec<DadosProcesso> = Vec::new();
        for pid in pids_principais {
            let Some(processo_principal) = sistema.process(sysinfo::Pid::from_u32(pid)) else {
                continue;
            };

            let mut ram_total = 0.0;
            let mut cpu_total = 0.0;
            let mut nome_app = corretor_nome(&processo_principal.name().to_string_lossy());
            for pid_processo in pids_da_arvore(pid) {
                if let Some(processo) = sistema.process(sysinfo::Pid::from_u32(pid_processo)) {
                    ram_total += processo.memory() as f64;
                    cpu_total += processo.cpu_usage() as f64;
                    if nome_app.to_string() == "bash"{
                        if nome_app.to_string() != processo.name().to_string_lossy().to_string(){
                            nome_app=corretor_nome(&processo_principal.name().to_string_lossy());
                        }
                    }
                }

            }
            let mut icone="".to_string();
            if resolver_icone(nome_app.to_string()) != ""{icone=resolver_icone(nome_app.to_string());}
            if snap::icone(pid) != ""{icone=snap::icone(pid);}

            if pids_jogos_steam.contains(&pid){
                lista.push(DadosProcesso {
                    icone: steam::icone(pid),
                    pid,
                    nome: steam::nome(pid),
                    ram: ram_total,
                    cpu: cpu_total / quantidade_cpus,
            });
            }
            else{
            lista.push(DadosProcesso {
                icone: icone,
                pid,
                nome: maiuscula_inicial(&nome_app.to_string()),
                ram: ram_total,
                cpu: cpu_total / quantidade_cpus,
            });
        }}
        let _ = app.emit("DadosMonitorProcessos", lista);

    }
}
