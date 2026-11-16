import { useState } from 'react';
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { AppWindow, Cpu, Gpu, MemoryStick } from 'lucide-react';
import './Processos.css';



function formatarMemoria(valor) {
  if (valor === null || valor === undefined || valor === '') return '—';

  const bytes = Number(valor);
  if (!Number.isFinite(bytes) || bytes < 0) return '—';

  const MB = 1_000_000;
  const GB = 1_000_000_000;

  const formatar = (numero) =>
    numero.toLocaleString('pt-BR', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    });

  if (bytes >= GB) {
    return `${formatar(bytes / GB)} GB`;
  }

  return `${formatar(bytes / MB)} MB`;
}



function formatarPercentual(valor) {
  if (valor === null || valor === undefined || valor === '') return '—';
  return typeof valor === 'number' ? `${valor.toFixed(1)}%` : valor;
}



function Processos({ processos = [], onEncerrar }) {
  const [pidSelecionado, setPidSelecionado] = useState(null);
  const processoSelecionado = processos.find((processo) => processo.pid === pidSelecionado);

  return (
    <main className="pagina-processos">
      <section className="processos-secao" aria-labelledby="processos-titulo">
        <header className="processos-cabecalho">
          <div>
            <span className="processos-rotulo">GERENCIAMENTO</span>
            <h2 id="processos-titulo">Processos em execução</h2>
            <p>Selecione um aplicativo para consultar e encerrar.</p>
          </div>
          <span className="processos-contagem">
            {processos.length} {processos.length === 1 ? 'processo' : 'processos'}
          </span>
        </header>

        <div className="processos-tabela-rolagem">
          <table className="processos-tabela">
            <thead>
              <tr>
                <th scope="col">Aplicativo</th>
                <th scope="col"><MemoryStick size={14} aria-hidden="true" /> RAM</th>
                <th scope="col"><Cpu size={14} aria-hidden="true" /> CPU</th>
                <th scope="col"><Gpu size={14} aria-hidden="true" /> GPU</th>
              </tr>
            </thead>
            <tbody>
              {processos.length === 0 ? (
                <tr>
                  <td className="processos-vazio" colSpan={4}>
                    A lista de processos pode estar carregando ou vazia.
                  </td>
                </tr>
              ) : (
                processos.map((processo) => {
                  const selecionado = processo.pid === pidSelecionado;

                  return (
                    <tr style={{"cursor":"pointer"}} 
                        onClick={() => setPidSelecionado(selecionado ? null : processo.pid)}
                        className={selecionado ? 'processo-linha processo-linha--selecionado' : 'processo-linha'} key={processo.pid}>
                      <td >
                        <button
                          className="processo-aplicativo"
                          type="button"
                          aria-pressed={selecionado}
                          onClick={() => setPidSelecionado(selecionado ? null : processo.pid)}
                        >
                          <span className="processo-icone">
                            {processo.icone != ""
                              ? <img src={convertFileSrc(processo.icone)} alt="" />
                              : <AppWindow size={17} aria-hidden="true" />}
                          </span>
                          <span className="processo-identificacao">
                            <strong>{processo.nome}</strong>
                            <small>PID {processo.pid}</small>
                          </span>
                        </button>
                      </td>
                      <td className="processo-valor">{formatarMemoria(processo.ram)}</td>
                      <td className="processo-valor">{formatarPercentual(processo.cpu)}</td>
                      <td className="processo-valor">{formatarPercentual(processo.gpuPercentual)}</td>
                    </tr>
                  );
                })
              )}
            </tbody>
          </table>
        </div>

        <footer className="processos-rodape">
          <span className="processos-selecao-info">
            {processoSelecionado ? `Selecionado: ${processoSelecionado.nome}` : 'Nenhum aplicativo selecionado'}
          </span>
          <button
            className="processos-botao-encerrar"
            type="button"
            disabled={!processoSelecionado}
            onClick={async () => {
              try {
                await invoke("encerrar_app", {
                  pid: processoSelecionado.pid,
                });
              } catch (erro) {
                console.error("Não foi possível encerrar o aplicativo:", erro);
              }
            }}
          >
            Encerrar aplicativo
          </button>
        </footer>
      </section>
    </main>
  );
}

export default Processos;
