use crate::modelos::InfoRPanel;
use std::sync::LazyLock;


pub(crate) static INFO_RPANEL: LazyLock<InfoRPanel> = LazyLock::new(|| InfoRPanel {
    versao: "2.0.0".to_string(),
    data_versao: "00/00/0000".to_string(),
    dev: "Bruno Barela Fernandes".to_string(),
    nome_link_git_hub: "GitHub RPanel".to_string(),
    endereco_link_git_hub: "https://github.com/BrunoBarelaFernandes/RPanel".to_string(),
    licenca: "GPL-3.0".to_string(),
});
