package one.tesseract.devwallet.ui.settings.home

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import one.tesseract.devwallet.ui.util.call

class HomeViewModel : ViewModel() {

    private val _text = MutableLiveData<String>().apply {
        value = "This is home Fragment"
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