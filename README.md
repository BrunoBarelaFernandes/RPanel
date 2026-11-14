# RPanel

**Monitoramento de recursos e gerenciamento de processos para Linux.**

O RPanel é uma aplicação desktop desenvolvida para monitorar os recursos do sistema e acompanhar os processos em execução, oferecendo informações sobre hardware, sistema operacional e utilização de recursos em uma interface gráfica.

## Funcionalidades

### Monitoramento de recursos

- Monitoramento de CPU.
- Monitoramento de memória RAM.
- Monitoramento de armazenamento.
- Monitoramento de rede.
- Monitoramento de GPU e VRAM, quando disponíveis.
- Indicador de conexão com a internet.

### Informações do sistema

- Informações sobre o sistema operacional.
- Modelo e fabricante da placa-mãe.
- Informações sobre o processador e memória.
- Identificação do ambiente gráfico X11 ou Wayland.
- Informações de firmware.
- Listagem de discos e suas informações.
- Tempo de atividade do sistema.
- Data e horário.

### Gerenciamento de processos

- Listagem de processos e aplicativos em execução.
- Identificação de aplicativos e seus ícones.
- Monitoramento de consumo de CPU e memória.
- Informações de utilização de disco e rede.
- Possibilidade de finalizar processos.

### Steam(Experimental ainda)

- O RPanel possui mecanismos de identificação para  jogos da Steam.



## Compatibilidade

O RPanel foi desenvolvido para Linux e testado em diferentes distribuições e ambientes gráficos. A grande maioria foi virtualizada.

| Distribuição           | VM  | Resultado                         |
| ---------------------- | --- | --------------------------------- |
| Ubuntu 26.04           | Não | Testado                           |
| Kubuntu 26             | Sim | Testado                           |
| Lubuntu 26             | Sim | Testado                           |
| Xubuntu 26             | Sim | Testado                           |
| Ubuntu MATE            | Sim | Compatibilidade parcial           |
| Linux Mint 22 MATE     | Sim | Pendente                          |
| Linux Mint 22 Cinnamon | Sim | Compatível, com bugs na interface |
| Zorin OS               | Sim | Testado                           |
| elementary OS          | Sim | Testado                           |
| Pop!\_OS               | Sim | Testado                           |
| Debian 13              | Sim | Testado                           |
| Kali Linux             | Sim | Testado                           |

A compatibilidade pode variar conforme o ambiente gráfico, os serviços de sessão e a configuração do sistema.

## Tecnologias utilizadas

- **Rust:** processamento e integração com o sistema operacional.
- **React:** construção da interface.
- **Tauri 2:** estrutura da aplicação desktop.

## Instalação

Acesse a seção [Releases](https://github.com/BrunoBarelaFernandes/RPanel/releases) para encontrar as versões disponibilizadas do RPanel.

Baixe o pacote correspondente à sua distribuição Linux e realize a instalação.



## Visão do futuro

O RPanel ainda está em suas versões iniciais de desenvolvimento. As funcionalidades e recursos atualmente disponíveis representam apenas o começo do projeto.

A implementação de todas as funcionalidades e ideias planejadas está prevista para a versão **3.0.0**, quando o RPanel deverá alcançar uma experiência mais completa, robusta e próxima da visão original do projeto.

Entre os planos futuros estão novos recursos de monitoramento, gráficos de desempenho em tempo real, melhorias na identificação de processos e uma experiência cada vez mais completa para o usuário.



## Desenvolvedor

**Bruno Barela Fernandes**

- GitHub: [BrunoBarelaFernandes](https://github.com/BrunoBarelaFernandes)
- Projeto: [RPanel](https://github.com/BrunoBarelaFernandes/RPanel)

## Versão atual

**1.0.0**
