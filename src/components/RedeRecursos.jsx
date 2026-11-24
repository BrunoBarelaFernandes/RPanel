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

function RedeRecursos({ recebimento = 0, envio = 0, labels }) {
  const baixando = Number(recebimento) > 0.0001;
  const enviando = Number(envio) > 0.0001;
  const download = formatarVelocidade(recebimento);
  const upload = formatarVelocidade(envio);

  return (
    <div className="rede-recursos">
      <div className={`rede-recursos_linha rede-recursos_linha_download ${baixando ? 'rede-recursos_linha_ativa' : ''}`}>
        <span className="rede-recursos_icone">
          <ArrowDownToLine size={17} strokeWidth={1.8} />
        </span>

        <div className="rede-recursos_descricao">
          <span className="rede-recursos_rotulo">{labels?.redeDownload}</span>
          <span className="rede-recursos_estado">
            {baixando ? labels?.redeLabelDownload : labels?.redeSemTrafego}
          </span>
        </div>

        <strong className="rede-recursos_valor">
          {download.valor}
          <small>{download.unidade}</small>
        </strong>
      </div>


      <div className={`rede-recursos_linha rede-recursos_linha_upload ${enviando ? 'rede-recursos_linha_ativa' : ''}`}>      <span className="rede-recursos_icone">
        <ArrowUpFromLine size={17} strokeWidth={1.8} />
      </span>

        <div className="rede-recursos_descricao">
          <span className="rede-recursos_rotulo">{labels?.redeUpload}</span>
          <span className="rede-recursos_estado">
            {enviando ? labels?.redeLabelUpload : labels?.redeSemTrafego}
          </span>
        </div>

        <strong className="rede-recursos_valor">
          {upload.valor}
          <small>{upload.unidade}</small>
        </strong>
      </div>


    </div>
  );
}

export default RedeRecursos;