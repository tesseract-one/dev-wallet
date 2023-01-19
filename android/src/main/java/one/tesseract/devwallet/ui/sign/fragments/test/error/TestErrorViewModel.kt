package one.tesseract.devwallet.ui.sign.fragments.test.error

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import one.tesseract.devwallet.entity.request.TestError

class TestErrorViewModel : ViewModel() {
    private val _request = MutableLiveData<TestError>().apply {
        value = null
    }
    val request: MutableLiveData<TestError> = _request
}