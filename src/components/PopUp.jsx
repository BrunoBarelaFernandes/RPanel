
import { useState, useEffect } from 'react';
import './PopUp.css';
import Alert from './Alert';
import { CircleCheckBig } from 'lucide-react';


function PopUp({ labels, aberto, nomeAplicativo, onConfirmar, onFechar}) {

    const [estado, setEstado] = useState('confirmacao');
    const [mensagemErro, setMensagemErro] = useState('');

    useEffect(() => {
        if (aberto) {
            setEstado('confirmacao');
            setMensagemErro('');
        }
    }, [aberto]);

    if (!aberto) return null;

    async function encerrarAplicativo() {
        setEstado('carregando');

        try {
            const resultado = await onConfirmar();

            if (resultado === false) {
                throw new Error(labels?.popupTitulo2Erro);
            }

            setEstado('sucesso');
        } catch (erro) {
            setMensagemErro(
                erro?.message || labels?.popupDescricaoErro
            );

            setEstado('erro');
        }
    }

    function fechar() {
        if (estado === 'carregando') return;

        onFechar();
    }

    return (
        <div className="popup-janela">
            <div className="popup-container">

                {estado === 'confirmacao' && (
                    <>
                        <div className="popup-icone aviso"><span><Alert tamanho={50} /></span></div>

                        <h2>{labels?.popupTitulo}</h2>

                        <p>
                            {labels?.popupTitulo2}
                            <strong> {nomeAplicativo}</strong>?
                        </p>

                        <p className="popup-detalhe">
                            {labels?.popupDescricao}
                        </p>

                        <div className="popup-botoes">
                            <button
                                className="popup-btn cancelar"
                                onClick={fechar}
                            >
                                {labels?.popupBotaoCancelar}
                            </button>

                            <button
                                className="popup-btn encerrar"
                                onClick={encerrarAplicativo}
                            >
                                {labels?.popupBotaoEncerrar}
                            </button>
                        </div>
                    </>
                )}

                {estado === 'carregando' && (
                    <div className="popup-carregando">
                        <div className="popup-spinner"></div>

                        <h2>{labels?.popupTituloAguarde}</h2>

                        <p>
                            {labels?.popupDescricaoAguarde}
                            <strong> {nomeAplicativo}</strong>...
                        </p>
                    </div>
                )}
                {estado === 'sucesso' && (
                    <>
                        <div className="popup-icone sucesso"><CircleCheckBig size={50} /></div>

                        <h2>{labels?.popupTituloSucesso}</h2>
                        
                {
                (() => {
                    const sucesso = labels?.popupDescricaoSucesso.split("{}");
                    return (
                    <>
                    {sucesso?.[0]}
                    <strong>{nomeAplicativo}</strong>
                    {sucesso?.[1]}
                    </>
                    );
                    })()
                }



                        <div className="popup-botoes">
                            <button
                                className="popup-btn confirmar"
                                onClick={fechar}
                            >
                                {labels?.popupBotaoOk}
                            </button>
                        </div>
                    </>
                )}

                {estado === 'erro' && (
                    <>
                        <div className="popup-icone erro"><span><Alert tamanho={50} /></span></div>

                        <h2>{labels?.popupTituloErro}</h2>

                        <p>
                            {labels?.popupTitulo2Erro}
                            <strong> {nomeAplicativo}</strong>
                        </p>

                        <p className="popup-detalhe">
                            {mensagemErro}
                        </p>

                        <div className="popup-botoes">
                            <button
                                className="popup-btn confirmar"
                                onClick={fechar}
                            >
                                {labels?.popupBotaoOk}
                            </button>
                        </div>
                    </>
                )}

            </div>
        </div>
    );
}


export default PopUp;