use genpdfi::{Document, elements, fonts, SimplePageDecorator};
use chrono::Local;
use crate::MONITOR;
use crate::modelos::Relatorio;
use tauri::Manager;
use crate::{
    gpu::consultar_gpu,
    dados_sobre::info,
    ram::ram_dados,
    monitor::tempo_sistema,
    idioma::idioma,
    disco::disco_os_data,
};

fn formatar_memoria(valor: Option<f64>) -> String {
    let Some(bytes) = valor else {
        return "—".into();
    };

    if !bytes.is_finite() || bytes < 0.0 {
        return "—".into();
    }

    const MB: f64 = 1_000_000.0;
    const GB: f64 = 1_000_000_000.0;

    let (numero, unidade) = if bytes >= GB {
        (bytes / GB, "GB")
    } else {
        (bytes / MB, "MB")
    };

    let formatado = numero.trunc().to_string().replace('.', ",");

    format!("{} {}", formatado, unidade)
}

fn tabela(tamanho: Vec<usize>, coluna: Vec<String>, dados: Vec<Vec<String>>) -> elements::TableLayout {

    let mut tabela = elements::TableLayout::new(tamanho);

    tabela.set_cell_decorator(
        elements::FrameCellDecorator::new(true, true, false)
    );

    let mut cabecalho = tabela.row();

    for col in coluna {
        cabecalho = cabecalho.element(
            elements::Paragraph::new(format!(" {}", col))
        );
    }

    cabecalho.push().unwrap();

    for linha in dados {
        let mut nova_linha = tabela.row();

        for valor in linha {
            nova_linha = nova_linha.element(
                elements::Paragraph::new(format!(" {}", valor))
            );
        }

        nova_linha.push().unwrap();
    }

    tabela
}

fn relatorio(app: tauri::AppHandle, dados: Relatorio) -> Result<(), Box<dyn std::error::Error>> {

    let caminho = app.path()
    .resource_dir().expect("ERRO: Caminho config")
    .join("RPanelData")
    .join("assets")
    .join("fontes");

    let fonte = fonts::from_files(caminho, "LiberationSans", None,)?;


    let mut doc = Document::new(fonte);

    doc.set_font_size(9);

    let mut decorador = SimplePageDecorator::new();
    decorador.set_margins(10);
    doc.set_page_decorator(decorador);


    doc.push(elements::Paragraph::new(
        dados.labels.sobre.parte_sistema_titulo.to_uppercase()
    ));

    let tabela_sistema = tabela(
        vec![1, 2],
        vec![dados.labels.sobre.parte_sobre_relatorio_propriedade.into(), dados.labels.sobre.parte_sobre_relatorio_valor.into()],
        vec![
            vec![
                dados.labels.sobre.parte_sistema_os.clone(),
                format!("{} {}", dados.dados_sobre.nome_os, dados.dados_sobre.versao_os)
            ],
            vec![
                dados.labels.sobre.parte_sistema_nome_host.clone(),
                dados.dados_sobre.nome_host.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_kernel.clone(),
                dados.dados_sobre.kernel.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_sistema_janelas.clone(),
                dados.dados_sobre.sistema_janelas.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_ambiente_grafico.clone(),
                dados.dados_sobre.ambiente_grafico.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_idioma.clone(),
                dados.dados_sobre.idioma.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_tempo_sistema.clone(),
                dados.dados_tempo_os.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_data_hora.clone(),
                dados.dados_data_hora.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_endereco_mac.clone(),
                dados.dados_sobre.mac.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_endereco_gateway.clone(),
                dados.dados_sobre.gateway.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_endereco_ip.clone(),
                dados.dados_sobre.ip.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_interface_rede.clone(),
                dados.dados_sobre.interface_rede.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_placa_mae.clone(),
                format!("{} {}", dados.dados_sobre.fabricante_placa_mae, dados.dados_sobre.modelo_placa_mae)
            ],
            vec![
                dados.labels.sobre.parte_sistema_versao_firmware.clone(),
                dados.dados_sobre.versao_firmware.clone()
            ],
            vec![
                dados.labels.sobre.parte_sistema_monitor.clone(),
                dados.dados_sobre.monitor.clone()
            ],
        ]
    );

    doc.push(tabela_sistema);


    doc.push(elements::Paragraph::new(""));
    doc.push(elements::Paragraph::new(
        dados.labels.sobre.parte_hardware_titulo.to_uppercase()
    ));

    let tabela_hardware = tabela(
        vec![1, 2],
        vec![dados.labels.sobre.parte_sobre_relatorio_componente.into(), dados.labels.sobre.parte_sobre_relatorio_especificacao.into()],
        vec![
            vec![
                dados.labels.sobre.parte_hardware_disco.clone(),
                format!(
                    "{} - {}",
                    dados.dados_disco.tipo,
                    formatar_memoria(Some(dados.dados_disco.total)),
                )
            ],
            vec![
                dados.labels.sobre.parte_hardware_ram.clone(),
                format!("{} - {}/{} {} - {} - {}",
                dados.dados_info_ram.ddr,
                dados.dados_info_ram.soquetes,
                dados.dados_info_ram.instalados,
                dados.labels.sobre.parte_hardware_ram_pentes,
                dados.dados_info_ram.velocidade,
                formatar_memoria(dados.dados_ram_total.parse::<f64>().ok())           
            )
            ],
            vec![
                dados.labels.sobre.parte_hardware_vram.clone(),
                format!(
                    "{}: {} - {}: {}",
                    dados.labels.sobre.parte_hardware_vram_fabricante,
                    dados.dados_gpu.fabricante,
                    dados.labels.sobre.parte_hardware_vram_total,
                    formatar_memoria(Some(dados.dados_gpu.memoria_total)),
                )
            ],
            vec![
                dados.labels.sobre.parte_hardware_gpu.clone(),
                dados.dados_gpu.modelo.clone()
            ],
        ]
    );

    doc.push(tabela_hardware);

 

    if let Some(pasta) = rfd::FileDialog::new().pick_folder() {
        let caminho = pasta.join(format!("{}-rpanel.pdf", dados.labels.sobre.parte_sobre_relatorio_nome));
        doc.render_to_file(caminho)?;
    }

    Ok(())
}


#[tauri::command]
pub(crate) async fn criar_relatorio(app: tauri::AppHandle) -> Result<(), String> {

    tauri::async_runtime::spawn_blocking(move || {
            let dados_sobre = info();

        let dados = {
            let mut monitor = MONITOR.get().unwrap().lock().unwrap();

            monitor.sistema.refresh_memory();

            let dados_disco = disco_os_data(&monitor.discos);

            let labels = idioma(app.clone());


            let dados_tempo_os = tempo_sistema(labels.clone());

            let dados_data_hora = Local::now()
                .format(&labels.sobre.parte_sistema_data_hora_formato)
                .to_string();

            let dados_ram_total = monitor.sistema.total_memory().to_string();
            let dados_info_ram = ram_dados();
            let dados_gpu = consultar_gpu();
            Relatorio {
                dados_sobre,
                dados_tempo_os,
                dados_data_hora,
                dados_ram_total,
                dados_info_ram,
                dados_gpu,
                dados_disco,
                labels,
            }
        };

        relatorio(app.clone(), dados);

    })
    .await
    .map_err(|erro| erro.to_string())
}