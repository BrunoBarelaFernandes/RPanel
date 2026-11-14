
use std::process::Command;
use std::fs;



fn pid_service(unidade: &str) -> u32 {
    let saida = Command::new("systemctl")
        .args([
            "--user",
            "show",
            unidade,
            "--property=MainPID",
            "--value",
        ])
        .output();

    match saida {
        Ok(saida) if saida.status.success() => {
            String::from_utf8_lossy(&saida.stdout)
                .trim()
                .parse::<u32>()
                .unwrap_or(0)
        }
        _ => 0,
    }
}



fn desktops() -> Vec<String> {
    let diretorios = [
        "/usr/share/applications",
        "/usr/local/share/applications",
        &format!("{}/.local/share/applications", std::env::var("HOME").unwrap_or_default()),
    ];

    let mut nomes = Vec::new();

    for diretorio in diretorios {
        let Ok(arquivos) = fs::read_dir(diretorio) else {
            continue;
        };

        for arquivo in arquivos.flatten() {
            let caminho = arquivo.path();

            if caminho.extension().is_some_and(|ext| ext == "desktop") {
                if let Some(nome) = caminho.file_stem().and_then(|n| n.to_str()) {
                    nomes.push(nome.to_lowercase());
                }
            }
        }
    }

    nomes.sort();
    nomes.dedup();

    nomes
}




pub(crate) fn pid_desktop() -> Vec<u32> {
    let desktop = desktops();
    let mut lista: Vec<u32> = Vec::new();

    let saida = Command::new("ps")
        .args(["-eo", "pid,comm"])
        .output();

    if let Ok(saida) = saida {
        if saida.status.success() {
            let texto = String::from_utf8_lossy(&saida.stdout);

            for linha in texto.lines().skip(1) {
                let mut dados = linha.split_whitespace();

                let Some(pid) = dados.next() else {continue;};

                let Some(nome) = dados.next() else {continue;};

                if desktop.contains(&nome.to_lowercase()) {
                    if let Ok(pid) = pid.parse::<u32>() {
                        lista.push(pid);
                    }
                }
            }
        }
    }

    lista
}



pub(crate) fn pid(nome: &str)->u32{

        if let Some(unidade) = nome.strip_suffix(".scope") {
        if let Some(numero) = unidade.rsplit('-').next().and_then(|v| v.parse::<u32>().ok()) {
            return numero;
        }
    }
    pid_service(nome)
}