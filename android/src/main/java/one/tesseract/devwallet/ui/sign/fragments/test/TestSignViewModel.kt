package one.tesseract.devwallet.ui.sign.fragments.test

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class TestSignViewModel : ViewModel() {
    private val _transaction = MutableLiveData<String>().apply {
        value = "Transaction"
    }
    val transaction: LiveData<String> = _transaction
}