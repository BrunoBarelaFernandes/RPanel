use std::{process::Command};
use std::fs;
use std::env;
use std::path::PathBuf;
use crate::processos::{pids_da_arvore, resolver_icone};

pub(crate) fn pid(pid_steam: u32) -> Option<u32> {
    pids_da_arvore(pid_steam).into_iter().find_map(|pid| {
        if pid == pid_steam {
            return None;
        }

        let comando = fs::read(format!("/proc/{pid}/cmdline")).ok()?;
        let comando = String::from_utf8_lossy(&comando);

        (comando.contains("SteamLaunch") && comando.contains("AppId="))
            .then_some(pid)
    })
}



pub(crate) fn appid(pid: u32) -> Option<String> {
    let comando = fs::read(format!("/proc/{pid}/cmdline")).ok()?;
    let comando = String::from_utf8_lossy(&comando);

    if !comando.contains("SteamLaunch") {
        return None;
    }

    let inicio = comando.find("AppId=")? + "AppId=".len();
    let appid: String = comando[inicio..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();

    if appid.is_empty() {
        None
    } else {
        Some(appid)
    }
}


pub(crate) fn nome(pid_raiz: u32) -> String {
    let Ok(saida) = Command::new("pstree")
        .args(["-T", "-p", "-l"])
        .arg(pid_raiz.to_string())
        .output()
    else {
        return String::new();
    };

    if !saida.status.success() {
        return String::new();
    }

    let arvore = String::from_utf8_lossy(&saida.stdout);
    let Ok(home) = env::var("HOME") else {
        return String::new();
    };

    for trecho in arvore.split('(').skip(1) {
        let Some((pid_texto, _)) = trecho.split_once(')') else {
            continue;
        };
        let Ok(pid) = pid_texto.parse::<u32>() else {
            continue;
        };

        let Ok(comando) = fs::read(format!("/proc/{pid}/cmdline")) else {
            continue;
        };
        let comando = String::from_utf8_lossy(&comando);

        if !comando.contains("SteamLaunch") {
            continue;
        }

        let Some(inicio) = comando.find("AppId=").map(|i| i + "AppId=".len()) else {
            continue;
        };
        let appid: String = comando[inicio..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();

        if appid.is_empty() {
            continue;
        }

        let manifesto = PathBuf::from(&home)
            .join(".local/share/Steam/steamapps")
            .join(format!("appmanifest_{appid}.acf"));

        let Ok(conteudo) = fs::read_to_string(manifesto) else {
            continue;
        };

        for linha in conteudo.lines() {
            let campos: Vec<_> = linha.split('"').collect();
            if campos.get(1) == Some(&"name") {
                if let Some(nome) = campos.get(3) {
                    return nome.to_string();
                }
            }
        }
    }

    String::new()
}



pub(crate) fn icone(pid_raiz: u32) -> String {
    let Some(appid) = pids_da_arvore(pid_raiz)
        .into_iter()
        .find_map(appid)
    else {
        return String::new();
    };

    let nome = nome(pid_raiz);

    let Ok(home) = env::var("HOME") else {
        return resolver_icone(nome);
    };

    let pasta_jogo = PathBuf::from(home)
        .join(".local/share/Steam/appcache/librarycache")
        .join(&appid);

    let icone = fs::read_dir(&pasta_jogo)
        .ok()
        .and_then(|arquivos| {
            arquivos
                .flatten()
                .map(|arquivo| arquivo.path())
                .find(|caminho| {
                    let stem = caminho.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let extensao = caminho.extension().and_then(|s| s.to_str()).unwrap_or("");

                    caminho.is_file()
                        && stem.len() == 40
                        && stem.bytes().all(|b| b.is_ascii_hexdigit())
                        && (extensao == "jpg" || extensao == "png")
                })
        });

    if let Some(caminho) = icone {
        return fs::canonicalize(&caminho)
            .unwrap_or(caminho)
            .to_string_lossy()
            .into_owned();
    }

    let logo = pasta_jogo.join("logo.png");
    if logo.is_file() {
        return fs::canonicalize(&logo)
            .unwrap_or(logo)
            .to_string_lossy()
            .into_owned();
    }

    resolver_icone(nome)
}




