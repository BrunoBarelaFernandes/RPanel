
import { useState, useEffect } from 'react';
import './PopUpDoacao.css';
import { HandCoins } from 'lucide-react';

function PopUpDoacao({ labels, aberto, onFechar, onConfirmar }) {
    const [formaPagamento, setFormaPagamento] = useState('');
    const [confirmarMeio, setConfirmarMeio] = useState('');

    useEffect(() => {
        if (aberto) {
            setFormaPagamento('pix');
            setConfirmarMeio('');
        }
    }, [aberto]);

    function confirmarPagamento() {
        if (formaPagamento === 'pix') {
            setConfirmarMeio('pix');
        } else {
            onConfirmar(formaPagamento);
            onFechar();
        }
    }

    if (!aberto) return null;

    return (
        <div className='container'>
            <div className="container_popup">
                <HandCoins className='popup-icone'/>

                <br/>

                {confirmarMeio === 'pix' ? (
                    <>
                        <div className='popup-detalhe'>
                            {labels?.parteSobreDoacaoPopupDescricaoQrPix}
                        </div>

                        <img
                            src="/QRPIX3.png"
                            alt="QR Code Pix"
                            className="popup-qrcode"
                        />

                        <div className='popup-botoes'>
                            <button
                                className='popup-btn cancelar'
                                onClick={() => setConfirmarMeio('paypal')}
                            >
                                {labels?.parteSobreDoacaoPopupBotaoVoltar}
                            </button>

                            <button
                                className='popup-btn confirmar'
                                onClick={onFechar}
                            >
                                {labels?.parteSobreDoacaoPopupBotaoFechar}
                            </button>
                        </div>
                    </>
                ) : (
                    <>
                        <select
                            value={formaPagamento}
                            onChange={(e) => setFormaPagamento(e.target.value)}
                        >
                            <option value="pix">Pix</option>
                        </select>

                        <br/>

                        <div className='popup-detalhe'>
                            {labels?.parteSobreDoacaoPopupDescricao}
                        </div>

                        <div className='popup-botoes'>
                            <button
                                className='popup-btn confirmar'
                                onClick={confirmarPagamento}
                            >
                                {labels?.parteSobreDoacaoPopupBotaoConfirmar}
                            </button>

                            <button
                                className='popup-btn cancelar'
                                onClick={onFechar}
                            >
                                {labels?.parteSobreDoacaoPopupBotaoCancelar}
                            </button>
                        </div>
                    </>
                )}
            </div>
        </div>
    );
}

export default PopUpDoacao;
