
import { useState, useEffect } from 'react';
import './PopUpIdiomas.css';
import { Languages, CircleCheckBig } from 'lucide-react';
import { invoke } from "@tauri-apps/api/core";

function PopUpIdiomas({
    labels,
    aberto,
    idiomasDisponiveis = [],
    onFechar,
    att
}) {
    const [idiomaSelecionado, setIdiomaSelecionado] = useState(labels?.infoIdioma?.arquivo);
    const [estado, setEstado] = useState('selecao');
    const [mensagemErro, setMensagemErro] = useState('');

    useEffect(() => {
        if (aberto) {
            setEstado('selecao');
            setMensagemErro('');
            setIdiomaSelecionado(labels?.infoIdioma?.arquivo);
        }
    }, [aberto]);

    async function attIdioma(i){
    invoke('mudar_idioma', { idioma: i });
    att();
    onFechar();
    }

    if (!aberto) return null;

    return (
        <div className='container'>
            <div className="container_popup">
                <Languages className='popup-icone'/>
                <br/>
                <select
                value={idiomaSelecionado}
                onChange={(e) => setIdiomaSelecionado(e.target.value)}
                >
                    {idiomasDisponiveis.map((idioma) => (
                        <option key={idioma}>{idioma}</option>
                    ))}

                </select>
                <br/>
                <div className='popup-detalhe'>{labels?.infoIdioma?.idioma}</div>
                <div className='popup-botoes'>
                    <button className='popup-btn confirmar' onClick={()=>{attIdioma(idiomaSelecionado)}}>{labels.layout.popUpIdiomaBotaoAplicar}</button>
                    <button className='popup-btn cancelar' onClick={() => {onFechar()}}>{labels.layout.popUpIdiomaBotaoCancelar}</button>

                </div>
            </div>
        
        </div>
    
    );
}

export default PopUpIdiomas;
