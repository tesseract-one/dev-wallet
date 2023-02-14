package one.tesseract.devwallet.ui.sign.fragments.substrate.sign

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import one.tesseract.devwallet.entity.request.SubstrateSign

class SubstrateSignViewModel : ViewModel() {
    private val _request = MutableLiveData<SubstrateSign>().apply {
        value = null
    }
    val request: MutableLiveData<SubstrateSign> = _request
}