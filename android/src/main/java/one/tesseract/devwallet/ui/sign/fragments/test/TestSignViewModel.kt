package one.tesseract.devwallet.ui.sign.fragments.test

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import one.tesseract.devwallet.entity.request.TestSign

class TestSignViewModel : ViewModel() {
    private val _request = MutableLiveData<TestSign>().apply {
        value = null
    }
    val request: MutableLiveData<TestSign> = _request
}