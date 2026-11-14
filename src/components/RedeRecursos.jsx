import { ArrowDownToLine, ArrowUpFromLine } from 'lucide-react';
import './RedeRecursos.css';

function formatarVelocidade(mbPorSegundo = 0) {
  const valor = Number(mbPorSegundo);

  if (!Number.isFinite(valor) || valor <= 0) {
    return { valor: '0', unidade: 'B/s' };
  }

  const bytesPorSegundo = valor * 1024 * 1024;

  if (bytesPorSegundo < 1024) {
    return {
      valor: bytesPorSegundo.toFixed(0),
      unidade: 'B/s',
    };
  }

  if (bytesPorSegundo < 1024 * 1024) {
    return {
      valor: (bytesPorSegundo / 1024).toFixed(2),
      unidade: 'KB/s',
    };
  }

  return {
    valor: (bytesPorSegundo / (1024 * 1024)).toFixed(2),
    unidade: 'MB/s',
  };
}

function RedeRecursos({ recebimento = 0, envio = 0 }) {
  const baixando = Number(recebimento) > 0.0001;
  const enviando = Number(envio) > 0.0001;
  const download = formatarVelocidade(recebimento);
  const upload = formatarVelocidade(envio);

  return (
    <div className="rede-recursos">
      <div
        className={`rede-recursos__linha rede-recursos__linha--download ${
          baixando ? 'rede-recursos__linha--ativa' : ''
        }`}
      >
        <span className="rede-recursos__icone" aria-hidden="true">
          <ArrowDownToLine size={17} strokeWidth={1.8} />
        </span>

        <div className="rede-recursos__descricao">
          <span className="rede-recursos__rotulo">Download</span>
          <span className="rede-recursos__estado">
            {baixando ? 'Recebendo dados' : 'Sem tráfego'}
          </span>
        </div>

        <strong className="rede-recursos__valor">
          {download.valor}
          <small>{download.unidade}</small>
        </strong>
      </div>

      <div
        className={`rede-recursos__linha rede-recursos__linha--upload ${
          enviando ? 'rede-recursos__linha--ativa' : ''
        }`}
      >
        <span className="rede-recursos__icone" aria-hidden="true">
          <ArrowUpFromLine size={17} strokeWidth={1.8} />
        </span>

        <div className="rede-recursos__descricao">
          <span className="rede-recursos__rotulo">Upload</span>
          <span className="rede-recursos__estado">
            {enviando ? 'Enviando dados' : 'Sem tráfego'}
          </span>
        </div>

        <strong className="rede-recursos__valor">
          {upload.valor}
          <small>{upload.unidade}</small>
        </strong>
      </div>
    </div>
  );
}

export default RedeRecursos;