import './StatusInternet.css';
import { Globe } from 'lucide-react';

function StatusInternet({ conectado = true, labels }) {
  const classeEstado = conectado
    ? 'status-internet--conectado'
    : 'status-internet--desconectado';

  return (
    <div
      className={`status-internet ${classeEstado}`}
      aria-label={`Internet ${conectado ? 'conectado' : 'sem acesso'}`}
    >

      <span className="status-internet-globo" aria-hidden="true">
        <span className="status-internet-aura" />
  
        <Globe className="status-internet-icone" size={21} strokeWidth={1.7} />
      </span>

      <span className="status-internet-texto">
        <span className="status-internet-rotulo">{labels?.statusInternetTitulo}</span>
        <strong>{conectado ? labels?.statusInternetConectado : labels?.statusInternetDesconectado}</strong>
      </span>

      <span className="status-internet-indicador" aria-hidden="true" />


    </div>
  );
}

export default StatusInternet;