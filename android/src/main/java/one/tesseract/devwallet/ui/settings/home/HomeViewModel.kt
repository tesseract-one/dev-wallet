package one.tesseract.devwallet.ui.settings.home

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import one.tesseract.devwallet.ui.util.call

class HomeViewModel : ViewModel() {

    private val _text = MutableLiveData<String>().apply {
        value = """
            Thanks for installing the Tesseract developer wallet!
            
            It's designed to help you test your dApp integration with Wallets through Tesseract protocol.
            
            Here, in the app itself, you won't find the balance or tokens like in the wallets designed to hold crypto. For the sake of simplicity, the app contains only settings to provide your testing private key (don't use the key where you keep your crypto!) and set up the "tesseract test" protocol if needed.
            
            Currently supported protocols:
            
            - Substrate/Polkadot
            - Tesseract Test protocol (to test tesseract connectivity)
        """
    }
    val text: LiveData<String> = _text

    private val _open = MutableLiveData<Unit>().apply {
        value = null
    }
    val open: LiveData<Unit> = _open

    fun showSign() {
        _text.value = "Sign"
        _open.call()
    }
}