use serde::{Serialize, Deserialize};
use sysinfo::{Components, Disks, Networks, System};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosSistemaRAM {
    pub(crate) ddr: String,
    pub(crate) soquetes: String,
    pub(crate) instalados: String,
    pub(crate) velocidade: String,
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosSobre {
    pub(crate) nome_os: String,
    pub(crate) versao_os: String,
    pub(crate) kernel: String,
    pub(crate) sistema_janelas: String,
    pub(crate) ambiente_grafico: String,
    pub(crate) nome_host: String,
    pub(crate) idioma: String,
    pub(crate) interface_rede: String,
    pub(crate) gateway: String,
    pub(crate) mac: String,
    pub(crate) ip: String,
    pub(crate) fabricante_placa_mae: String,
    pub(crate) modelo_placa_mae: String,
    pub(crate) versao_firmware: String,
    pub(crate) monitor: String,
    pub(crate) discos: ListaDisco,
    pub(crate) sobre_rpanel: InfoRPanel,

}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosProcesso {
    pub(crate) icone: String,
    pub(crate) pid: u32,
    pub(crate) nome: String,
    pub(crate) ram: f64,
    pub(crate) cpu: f64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosMonitor {
    pub(crate) cpu: DadosCPU,
    pub(crate) ram: DadosRam,
    pub(crate) disco: DadosDisco,
    pub(crate) rede: DadosRede,
    pub(crate) tempo_os: String,
    pub(crate) data_hora: String,

}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosGPU {
    pub fabricante: String,
    pub modelo: String,
    pub uso_percentual: u8,
    pub uso_percentual_memoria: u8,
    pub temperatura: f64,
    pub memoria_total: f64,
    pub memoria_usada: f64,
    pub memoria_restante: f64,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosCPU {
    pub(crate) modelo: String,
    pub(crate) frequencia: String,
    pub(crate) uso_percentual: u8,
    pub(crate) temperatura: u8,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosRam {
    pub(crate) total: f64,
    pub(crate) usada: f64,
    pub(crate) restante: f64,
    pub(crate) uso_percentual: u8,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosDisco {
    pub(crate) tipo: String,
    pub(crate) total: f64,
    pub(crate) usada: f64,
    pub(crate) restante: f64,
    pub(crate) uso_percentual: u8,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DadosRede {
    pub(crate) interface: String,
    pub(crate) download: f64,
    pub(crate) upload: f64,
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Disco {
    pub(crate) dispositivo: String,
    pub(crate) montagem: String,
    pub(crate) tipo: String,
    pub(crate) total: f64,
    pub(crate) usada: f64,
    pub(crate) restante: f64,
    pub(crate) uso_percentual: u8,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListaDisco {
    pub(crate) lista: Vec<Disco>,

}


pub(crate) struct DadosSysInfo {
    pub(crate) sistema: System,
    pub(crate) componentes: Components,
    pub(crate) discos: Disks,
    pub(crate) redes: Networks,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InfoRPanel {
    pub(crate) versao: String,
    pub(crate) data_versao: String,
    pub(crate) dev: String,
    pub(crate) nome_link_git_hub: String,
    pub(crate) endereco_link_git_hub: String,
    pub(crate) licenca: String,

}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) idioma: String,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Labels {
    pub(crate) info_idioma: InfoIdioma,
    pub(crate) layout: LayoutLabel,
    pub(crate) recurso: RecursoLabel,
    pub(crate) processo: ProcessoLabel,
    pub(crate) sobre: SobreLabel,
    pub(crate) aviso_valores: AvisoValoresLabel,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InfoIdioma {
    pub(crate) idioma: String,
    pub(crate) arquivo: String,
    pub(crate) versao: String,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LayoutLabel {
    pub(crate) titulo: String,
    pub(crate) menu_titulo: String,
    pub(crate) menu_item_recursos: String,
    pub(crate) menu_item_processos: String,
    pub(crate) menu_item_sobre: String,
    pub(crate) status_internet_titulo: String,
    pub(crate) status_internet_conectado: String,
    pub(crate) status_internet_desconectado: String,
    pub(crate) pop_up_idioma_botao_cancelar: String,
    pub(crate) pop_up_idioma_botao_aplicar: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecursoLabel {
    pub(crate) titulo: String,
    pub(crate) descricao: String,
    pub(crate) descricao2: String,

    pub(crate) cpu_titulo: String,
    pub(crate) cpu_titulo2: String,
    pub(crate) cpu_temperatura: String,
    pub(crate) cpu_detalhes: String,

    pub(crate) gpu_titulo: String,
    pub(crate) gpu_titulo2: String,
    pub(crate) gpu_temperatura: String,
    pub(crate) gpu_detalhes: String,

    pub(crate) vram_titulo: String,
    pub(crate) vram_titulo2: String,
    pub(crate) vram_detalhes: String,
    pub(crate) vram_label_usada: String,
    pub(crate) vram_label_livre: String,

    pub(crate) ram_titulo: String,
    pub(crate) ram_titulo2: String,
    pub(crate) ram_detalhes: String,
    pub(crate) ram_detalhes2: String,
    pub(crate) ram_label_usada: String,
    pub(crate) ram_label_livre: String,

    pub(crate) disco_titulo: String,
    pub(crate) disco_titulo2: String,
    pub(crate) disco_detalhes: String,
    pub(crate) disco_label_usada: String,
    pub(crate) disco_label_livre: String,

    pub(crate) rede_titulo: String,
    pub(crate) rede_detalhes: String,
    pub(crate) rede_detalhes2: String,
    pub(crate) rede_download: String,
    pub(crate) rede_label_download: String,
    pub(crate) rede_upload: String,
    pub(crate) rede_label_upload: String,
    pub(crate) rede_sem_trafego: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProcessoLabel {
    pub(crate) titulo: String,
    pub(crate) titulo2: String,
    pub(crate) descricao: String,
    pub(crate) processo_quantidade: String,
    pub(crate) processos_quantidade: String,
    pub(crate) titulo_tabela: String,
    pub(crate) titulo_tabela_cpu: String,
    pub(crate) titulo_tabela_ram: String,
    pub(crate) titulo_tabela_gpu: String,
    pub(crate) aplicativo_selecionado: String,
    pub(crate) aplicativo_selecionado2: String,
    pub(crate) botao_encerrar: String,
    pub(crate) aviso_sem_processo: String,

    pub(crate) popup_titulo: String,
    pub(crate) popup_titulo2: String,
    pub(crate) popup_descricao: String,
    pub(crate) popup_botao_cancelar: String,
    pub(crate) popup_botao_encerrar: String,

    pub(crate) popup_titulo_sucesso: String,
    pub(crate) popup_descricao_sucesso: String,
    pub(crate) popup_botao_ok: String,

    pub(crate) popup_titulo_erro: String,
    pub(crate) popup_titulo2_erro: String,
    pub(crate) popup_descricao_erro: String,

    pub(crate) popup_titulo_aguarde: String,
    pub(crate) popup_descricao_aguarde: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SobreLabel {
    pub(crate) parte_sistema_titulo: String,
    pub(crate) parte_sistema_titulo2: String,
    pub(crate) parte_sistema_descricao: String,

    pub(crate) parte_sistema_os: String,
    pub(crate) parte_sistema_nome_host: String,
    pub(crate) parte_sistema_kernel: String,
    pub(crate) parte_sistema_sistema_janelas: String,
    pub(crate) parte_sistema_ambiente_grafico: String,
    pub(crate) parte_sistema_idioma: String,

    pub(crate) parte_sistema_tempo_sistema: String,
    pub(crate) parte_sistema_tempo_sistema_label: String,
    pub(crate) parte_sistema_tempo_sistema_label2: String,
    pub(crate) parte_sistema_tempo_sistema_label3: String,

    pub(crate) parte_sistema_data_hora: String,
    pub(crate) parte_sistema_data_hora_formato: String,
    pub(crate) parte_sistema_endereco_mac: String,
    pub(crate) parte_sistema_endereco_gateway: String,
    pub(crate) parte_sistema_endereco_ip: String,
    pub(crate) parte_sistema_interface_rede: String,
    pub(crate) parte_sistema_placa_mae: String,
    pub(crate) parte_sistema_versao_firmware: String,
    pub(crate) parte_sistema_monitor: String,

    pub(crate) parte_hardware_titulo: String,
    pub(crate) parte_hardware_titulo2: String,
    pub(crate) parte_hardware_descricao: String,
    pub(crate) parte_hardware_cpu: String,
    pub(crate) parte_hardware_ram: String,
    pub(crate) parte_hardware_ram_pentes: String,
    pub(crate) parte_hardware_disco: String,
    pub(crate) parte_hardware_vram: String,
    pub(crate) parte_hardware_vram_fabricante: String,
    pub(crate) parte_hardware_vram_total: String,
    pub(crate) parte_hardware_vram_usada: String,
    pub(crate) parte_hardware_vram_livre: String,
    pub(crate) parte_hardware_gpu: String,

    pub(crate) parte_armazenamento_titulo: String,
    pub(crate) parte_armazenamento_titulo2: String,
    pub(crate) parte_armazenamento_descricao: String,
    pub(crate) parte_armazenamento_titulo_tabela: String,
    pub(crate) parte_armazenamento_titulo_tabela_tipo: String,
    pub(crate) parte_armazenamento_titulo_tabela_total: String,
    pub(crate) parte_armazenamento_titulo_tabela_usado: String,
    pub(crate) parte_armazenamento_titulo_tabela_livre: String,
    pub(crate) parte_armazenamento_marca_sistema_tabela: String,

    pub(crate) parte_sobre_titulo: String,
    pub(crate) parte_sobre_titulo2: String,
    pub(crate) parte_sobre_descricao: String,
    pub(crate) parte_sobre_versao: String,
    pub(crate) parte_sobre_data_versao: String,
    pub(crate) parte_sobre_dev: String,
    pub(crate) parte_sobre_licenca: String,
    pub(crate) parte_sobre_doacao_titulo: String,
    pub(crate) parte_sobre_doacao_texto: String,
    pub(crate) parte_sobre_doacao_popup_descricao: String,
    pub(crate) parte_sobre_doacao_popup_descricao_qr_pix: String,
    pub(crate) parte_sobre_doacao_popup_botao_confirmar: String,
    pub(crate) parte_sobre_doacao_popup_botao_cancelar: String,
    pub(crate) parte_sobre_doacao_popup_botao_voltar: String,
    pub(crate) parte_sobre_doacao_popup_botao_fechar: String,

    pub(crate) parte_sobre_relatorio_botao_titulo: String,
    pub(crate) parte_sobre_relatorio_botao_descricao: String,
    pub(crate) parte_sobre_relatorio_botao_gerar: String,
    pub(crate) parte_sobre_relatorio_propriedade: String,
    pub(crate) parte_sobre_relatorio_valor: String,
    pub(crate) parte_sobre_relatorio_componente: String,
    pub(crate) parte_sobre_relatorio_especificacao: String,
    pub(crate) parte_sobre_relatorio_nome: String,




}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AvisoValoresLabel {
    pub(crate) sem_gpu: String,
    pub(crate) sem_memoria_video: String,
    pub(crate) componente_sistema: String,
    pub(crate) sem_modelo_dados: String,
    pub(crate) sem_informacao: String,
    pub(crate) desconhecido: String,
    pub(crate) indefinido: String,
    pub(crate) indisponivel: String,
}


pub(crate) struct Relatorio{
    pub(crate) dados_sobre: DadosSobre,
    pub(crate) dados_tempo_os: String,
    pub(crate) dados_data_hora: String,
    pub(crate) dados_ram_total: String,
    pub(crate) dados_info_ram: DadosSistemaRAM,
    pub(crate) dados_gpu: DadosGPU,
    pub(crate) dados_disco: DadosDisco,
    pub(crate) labels: Labels,
}