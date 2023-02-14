package one.tesseract.devwallet.ui.sign.fragments.substrate.account

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import one.tesseract.devwallet.entity.request.SubstrateAccount

class SubstrateAccountViewModel : ViewModel() {
    private val _request = MutableLiveData<SubstrateAccount>().apply {
        value = null
    }
    val request: MutableLiveData<SubstrateAccount> = _request
}